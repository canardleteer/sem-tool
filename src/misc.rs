//! SPDX-License-Identifier: Apache-2.0
//! Copyright 2025 canardleteer
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//! http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.
//!
//! This is a bunch of last mile display + serialization logic.
use clap::ValueEnum;
use clap_mcp::{ClapMcpToolError, ClapMcpToolOutput, IntoClapMcpResult, IntoClapMcpToolError};
use core::fmt;
use serde::Serialize;
use std::process::{ExitCode, Termination};
use thiserror::Error;

use crate::results;

#[derive(Error, Debug)]
pub(crate) enum ApplicationError {
    /// We got invalid input.
    #[error("Invalid input (expected {expected:?}, got {found:?}")]
    InvalidArgument { expected: String, found: String },

    /// We were unable to prepare the output as requested.
    #[error("Failed to prepare output in this format {err:?}")]
    OutputFormatError { err: String },

    /// We failed some requirement while processing data.
    #[error("Failed a requirement {err:?}")]
    FailedRequirementError { err: String },
}

#[derive(ValueEnum, Clone, Debug)]
pub(crate) enum OutputFormat {
    Text,
    Yaml,
    Json,
}

impl OutputFormat {
    /// Format a subcommand result for CLI stdout. Not used for MCP (serialization handled there).
    pub(crate) fn format_result(
        &self,
        result: &SubcommandResult,
    ) -> Result<String, ApplicationError> {
        match self {
            OutputFormat::Text => Ok(result.to_string()),
            OutputFormat::Yaml => serde_yaml::to_string(result)
                .map_err(|e| ApplicationError::OutputFormatError { err: e.to_string() })
                .map(|s| format!("---\n{s}")),
            OutputFormat::Json => serde_json::to_string(result)
                .map_err(|e| ApplicationError::OutputFormatError { err: e.to_string() }),
        }
    }
}

/// Trait for subcommand output: types that can be written to stdout and report an exit code.
/// Each subcommand selects one of these types; the concrete type is preserved in [SubcommandResult].
#[allow(dead_code)] // bound for subcommand result types; used for documentation and future accessors
pub(crate) trait SubcommandOutput: fmt::Display + Serialize + Termination {}

impl<T: fmt::Display + Serialize + Termination> SubcommandOutput for T {}

/// Exit policy wrapper: normal (use the result's exit code) vs always success.
pub(crate) enum ExitOutcome {
    Normal(SubcommandResult),
    AlwaysSuccessful(SubcommandResult),
}

impl ExitOutcome {
    pub(crate) const fn new(output: SubcommandResult, hard_success: bool) -> Self {
        if hard_success {
            Self::AlwaysSuccessful(output)
        } else {
            Self::Normal(output)
        }
    }
}

impl Termination for ExitOutcome {
    fn report(self) -> ExitCode {
        match self {
            Self::Normal(result) => result.report(),
            Self::AlwaysSuccessful(_result) => ExitCode::SUCCESS,
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum SubcommandResult {
    /// Assertion by this program
    ComparisonStatement(results::ComparisonStatement),
    /// Ordered Map representation of versions
    OrderedVersionMap(results::OrderedVersionMap),
    /// Breakdown of version
    VersionExplanation(results::VersionExplanation),
    /// Flat list of versions
    FlatVersionsList(results::FlatVersionsList),
    /// Flat list of strings
    FlatStringList(results::FlatStringList),
    /// Results from a filter test
    FilterTestResult(results::FilterTestResult),
    /// Results from a test
    ValidateResult(results::ValidateResult),
    /// Just a plain version
    JustAVersion(results::VersionMutationResult),
}

impl From<results::ComparisonStatement> for SubcommandResult {
    fn from(value: results::ComparisonStatement) -> Self {
        Self::ComparisonStatement(value)
    }
}

impl From<results::OrderedVersionMap> for SubcommandResult {
    fn from(value: results::OrderedVersionMap) -> Self {
        Self::OrderedVersionMap(value)
    }
}
impl From<results::VersionExplanation> for SubcommandResult {
    fn from(value: results::VersionExplanation) -> Self {
        Self::VersionExplanation(value)
    }
}

impl From<results::FlatVersionsList> for SubcommandResult {
    fn from(value: results::FlatVersionsList) -> Self {
        Self::FlatVersionsList(value)
    }
}

impl From<results::FilterTestResult> for SubcommandResult {
    fn from(value: results::FilterTestResult) -> Self {
        Self::FilterTestResult(value)
    }
}

impl From<results::ValidateResult> for SubcommandResult {
    fn from(value: results::ValidateResult) -> Self {
        Self::ValidateResult(value)
    }
}

impl From<results::GenerateResult> for SubcommandResult {
    fn from(value: results::GenerateResult) -> Self {
        Self::FlatStringList(value.into())
    }
}

impl From<results::VersionMutationResult> for SubcommandResult {
    fn from(value: results::VersionMutationResult) -> Self {
        Self::JustAVersion(value)
    }
}

/// MCP tool response wrapper: explicit `ok` for exit-code-like semantics (Validate, FilterTest, Compare).
#[derive(Serialize)]
pub(crate) struct McpToolOutput {
    pub(crate) ok: bool,
    pub(crate) result: SubcommandResult,
}

/// Compute success for MCP: true for exit-code success, so clients get explicit semantics.
fn success_for_mcp(result: &SubcommandResult) -> bool {
    match result {
        SubcommandResult::ValidateResult(v) => v.success(),
        SubcommandResult::FilterTestResult(f) => f.success(),
        SubcommandResult::ComparisonStatement(c) => c.both_equal(),
        _ => true,
    }
}

impl IntoClapMcpResult for SubcommandResult {
    fn into_tool_result(self) -> std::result::Result<ClapMcpToolOutput, ClapMcpToolError> {
        let ok = success_for_mcp(&self);
        let wrapped = McpToolOutput { ok, result: self };
        serde_json::to_value(&wrapped)
            .map(ClapMcpToolOutput::Structured)
            .map_err(|e| ClapMcpToolError::text(e.to_string()))
    }
}

impl IntoClapMcpToolError for ApplicationError {
    fn into_tool_error(self) -> ClapMcpToolError {
        ClapMcpToolError::text(self.to_string())
    }
}

impl Termination for SubcommandResult {
    // NOTE(canardleteer): only expected to be called along certain code paths
    //                     (at least for now).
    fn report(self) -> ExitCode {
        match self {
            Self::ComparisonStatement(comparison_statement) => comparison_statement.report(),
            Self::FilterTestResult(filter_test_result) => filter_test_result.report(),
            Self::ValidateResult(validate_result) => validate_result.report(),
            _ => ExitCode::SUCCESS,
        }
    }
}

/// Display for subcommand result (delegates to inner type).
impl fmt::Display for SubcommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComparisonStatement(v) => {
                write!(f, "{}", v)
            }
            Self::OrderedVersionMap(v) => {
                write!(f, "{}", v)
            }
            Self::VersionExplanation(v) => {
                write!(f, "{}", v)
            }
            Self::FlatVersionsList(v) => {
                write!(f, "{}", v)
            }
            Self::FlatStringList(v) => {
                write!(f, "{}", v)
            }
            Self::FilterTestResult(v) => {
                write!(f, "{}", v)
            }
            Self::ValidateResult(v) => {
                write!(f, "{}", v)
            }
            Self::JustAVersion(v) => {
                write!(f, "{}", v)
            }
        }
    }
}

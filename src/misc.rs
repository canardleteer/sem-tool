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
    /// Single component selection result
    SelectResult(results::SelectResult),
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

impl From<results::SelectResult> for SubcommandResult {
    fn from(value: results::SelectResult) -> Self {
        Self::SelectResult(value)
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
            Self::SelectResult(select_result) => select_result.report(),
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
            Self::SelectResult(v) => {
                write!(f, "{}", v)
            }
        }
    }
}

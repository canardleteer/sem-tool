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

/// ApplicationTermination catches some of the awkward flagging around how we
/// determine our exit status.
pub(crate) enum ApplicationTermination {
    Normal(ApplicationOutput),
    AlwaysSuccessful(ApplicationOutput),
}

impl ApplicationTermination {
    pub(crate) fn new(output: ApplicationOutput, hard_success: bool) -> ApplicationTermination {
        if hard_success {
            ApplicationTermination::AlwaysSuccessful(output)
        } else {
            ApplicationTermination::Normal(output)
        }
    }
}

impl Termination for ApplicationTermination {
    fn report(self) -> ExitCode {
        match self {
            ApplicationTermination::Normal(application_output) => application_output.report(),
            ApplicationTermination::AlwaysSuccessful(_application_output) => ExitCode::SUCCESS,
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum ApplicationOutput {
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
}

impl From<results::ComparisonStatement> for ApplicationOutput {
    fn from(value: results::ComparisonStatement) -> Self {
        ApplicationOutput::ComparisonStatement(value)
    }
}

impl From<results::OrderedVersionMap> for ApplicationOutput {
    fn from(value: results::OrderedVersionMap) -> Self {
        ApplicationOutput::OrderedVersionMap(value)
    }
}
impl From<results::VersionExplanation> for ApplicationOutput {
    fn from(value: results::VersionExplanation) -> Self {
        ApplicationOutput::VersionExplanation(value)
    }
}

impl From<results::FlatVersionsList> for ApplicationOutput {
    fn from(value: results::FlatVersionsList) -> Self {
        ApplicationOutput::FlatVersionsList(value)
    }
}

impl From<results::FilterTestResult> for ApplicationOutput {
    fn from(value: results::FilterTestResult) -> Self {
        ApplicationOutput::FilterTestResult(value)
    }
}

impl From<results::ValidateResult> for ApplicationOutput {
    fn from(value: results::ValidateResult) -> Self {
        ApplicationOutput::ValidateResult(value)
    }
}

impl From<results::GenerateResult> for ApplicationOutput {
    fn from(value: results::GenerateResult) -> Self {
        ApplicationOutput::FlatStringList(value.into())
    }
}

impl Termination for ApplicationOutput {
    // NOTE(canardleteer): only expected to be called along certain code paths
    //                     (at least for now).
    fn report(self) -> ExitCode {
        match self {
            ApplicationOutput::ComparisonStatement(comparison_statement) => {
                comparison_statement.report()
            }
            ApplicationOutput::FilterTestResult(filter_test_result) => filter_test_result.report(),
            ApplicationOutput::ValidateResult(validate_result) => validate_result.report(),
            _ => ExitCode::SUCCESS,
        }
    }
}

/// Display for Application Output
///
/// While this exists, I'm tempted to just make the default YAML.
impl fmt::Display for ApplicationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationOutput::ComparisonStatement(v) => {
                write!(f, "{}", v)
            }
            ApplicationOutput::OrderedVersionMap(v) => {
                write!(f, "{}", v)
            }
            ApplicationOutput::VersionExplanation(v) => {
                write!(f, "{}", v)
            }
            ApplicationOutput::FlatVersionsList(v) => {
                write!(f, "{}", v)
            }
            ApplicationOutput::FlatStringList(v) => {
                write!(f, "{}", v)
            }
            ApplicationOutput::FilterTestResult(v) => {
                write!(f, "{}", v)
            }
            ApplicationOutput::ValidateResult(v) => {
                write!(f, "{}", v)
            }
        }
    }
}

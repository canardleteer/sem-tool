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

use crate::results::{
    BoundaryVersionResult, ComparisonStatement, FilterTestResult, FlatVersionsList, GenerateResult,
    OrderedVersionMap, SelectResult, ValidateResult, VersionExplanation, VersionMutationResult,
};

#[derive(Error, Debug)]
pub enum ApplicationError {
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
pub enum OutputFormat {
    Text,
    Yaml,
    Json,
}

/// Exit policy wrapper: normal (use the result's exit code) vs always success.
pub enum ExitOutcome {
    Normal(SubcommandResult),
    AlwaysSuccessful(SubcommandResult),
}

impl ExitOutcome {
    pub const fn new(output: SubcommandResult, hard_success: bool) -> Self {
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

macro_rules! subcommand_result {
    (
        enum $name:ident {
            $($variant:ident($ty:ty)),* $(,)?
        }
    ) => {
        #[derive(Serialize)]
        #[serde(untagged)]
        pub enum $name {
            $($variant($ty),)*
        }

        $(impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                Self::$variant(value)
            }
        })*

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$variant(inner) => write!(f, "{inner}"),)*
                }
            }
        }
    };
}

subcommand_result! {
    enum SubcommandResult {
        ComparisonStatement(ComparisonStatement),
        OrderedVersionMap(OrderedVersionMap),
        VersionExplanation(VersionExplanation),
        FlatVersionsList(FlatVersionsList),
        GenerateResult(GenerateResult),
        FilterTestResult(FilterTestResult),
        ValidateResult(ValidateResult),
        VersionMutation(VersionMutationResult),
        SelectResult(SelectResult),
        BoundaryVersionResult(BoundaryVersionResult),
    }
}

impl Termination for SubcommandResult {
    fn report(self) -> ExitCode {
        match self {
            Self::ComparisonStatement(s) => s.report(),
            Self::FilterTestResult(s) => s.report(),
            Self::ValidateResult(s) => s.report(),
            Self::SelectResult(s) => s.report(),
            Self::OrderedVersionMap(_)
            | Self::VersionExplanation(_)
            | Self::FlatVersionsList(_)
            | Self::GenerateResult(_)
            | Self::VersionMutation(_)
            | Self::BoundaryVersionResult(_) => ExitCode::SUCCESS,
        }
    }
}

pub fn emit(result: &SubcommandResult, format: OutputFormat) -> Result<(), ApplicationError> {
    match format {
        OutputFormat::Text => print!("{result}"),
        OutputFormat::Yaml => {
            println!("---");
            let yaml = serde_yaml::to_string(result)
                .map_err(|e| ApplicationError::OutputFormatError { err: e.to_string() })?;
            print!("{yaml}");
        }
        OutputFormat::Json => {
            let json = serde_json::to_string(result)
                .map_err(|e| ApplicationError::OutputFormatError { err: e.to_string() })?;
            println!("{json}");
        }
    }
    Ok(())
}

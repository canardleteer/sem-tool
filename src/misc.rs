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

macro_rules! subcommand_result {
    (
        enum $name:ident {
            $($variant:ident($ty:ty)),* $(,)?
        }
    ) => {
        #[derive(Serialize)]
        #[serde(untagged)]
        pub(crate) enum $name {
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

pub(crate) fn emit(
    result: &SubcommandResult,
    format: OutputFormat,
) -> Result<(), ApplicationError> {
    match format {
        OutputFormat::Text => print!("{result}"),
        OutputFormat::Yaml => {
            println!("---");
            let yaml = noyalib::to_string(result)
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

#[cfg(test)]
mod yaml_structure_tests {
    use super::SubcommandResult;
    use crate::results::{
        OrderedVersionMap, SelectResult, SemverComponent, ValidateResult, VersionExplanation,
        VersionMutationResult,
    };
    use semver::Version;

    fn parse_yaml_value(result: &SubcommandResult) -> serde_json::Value {
        let yaml = noyalib::to_string(result).unwrap();
        noyalib::from_str(&yaml).unwrap()
    }

    #[test]
    fn validate_result_yaml_structure() {
        let result = SubcommandResult::ValidateResult(ValidateResult::validate(
            "1.2.3".into(),
            false,
        ));
        let doc = parse_yaml_value(&result);
        assert_eq!(doc.get("valid").and_then(|v| v.as_bool()), Some(true));
    }

    #[test]
    fn version_mutation_result_yaml_structure() {
        let result = SubcommandResult::VersionMutation(VersionMutationResult {
            mutated_version: Version::parse("2.1.1").unwrap(),
        });
        let doc = parse_yaml_value(&result);
        assert_eq!(
            doc.get("mutated_version").and_then(|v| v.as_str()),
            Some("2.1.1")
        );
    }

    #[test]
    fn ordered_version_map_yaml_structure() {
        let mut versions = vec![
            Version::parse("1.0.0").unwrap(),
            Version::parse("2.0.0").unwrap(),
        ];
        let map = OrderedVersionMap::new(&mut versions, &None, false, false, false);
        let result = SubcommandResult::OrderedVersionMap(map);
        let doc = parse_yaml_value(&result);
        let versions = doc
            .get("versions")
            .and_then(|v| v.as_object())
            .expect("versions map");
        assert_eq!(versions.len(), 2);
        assert!(versions.contains_key("1.0.0"));
        assert!(versions.contains_key("2.0.0"));
    }

    #[test]
    fn version_explanation_yaml_structure() {
        let version = Version::parse("1.2.3-rc.0+build.1").unwrap();
        let result =
            SubcommandResult::VersionExplanation(VersionExplanation::from(&version));
        let doc = parse_yaml_value(&result);
        assert_eq!(doc.get("major").and_then(|v| v.as_u64()), Some(1));
        assert_eq!(doc.get("minor").and_then(|v| v.as_u64()), Some(2));
        assert_eq!(doc.get("patch").and_then(|v| v.as_u64()), Some(3));
        assert_eq!(
            doc.get("prerelease_string").and_then(|v| v.as_str()),
            Some("rc.0")
        );
        assert!(doc.get("prerelease").and_then(|v| v.as_array()).is_some());
        assert_eq!(
            doc.get("build_metadata_string").and_then(|v| v.as_str()),
            Some("build.1")
        );
        assert!(doc.get("build-metadata").and_then(|v| v.as_array()).is_some());
    }

    #[test]
    fn select_result_untagged_yaml_structure() {
        let inner = SelectResult::select("1.2.3", SemverComponent::Major, false, false).unwrap();
        let result = SubcommandResult::SelectResult(inner);
        let doc = parse_yaml_value(&result);
        assert_eq!(doc.get("value").and_then(|v| v.as_str()), Some("1"));
    }
}

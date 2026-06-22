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
use proptest::prelude::*;
use proptest_semver::*;
use semver::{BuildMetadata, Version, VersionReq};
use std::collections::HashMap;

mod common;
use common::subcommands::*;

use crate::common::common_cmd;

#[test]
fn cli_max_invalid_input() {
    let assert = common_cmd().arg(COMMAND_MAX).arg("a.b.c").assert();
    assert.append_context(COMMAND_MAX, "bad semver").failure();
}

#[test]
fn cli_max_basic_cases() {
    let assert = common_cmd()
        .arg("-o")
        .arg("text")
        .arg(COMMAND_MAX)
        .arg("1.0.0")
        .arg("2.0.0")
        .arg("1.5.0")
        .assert();
    assert
        .append_context(COMMAND_MAX, "simple max")
        .stdout("2.0.0\n")
        .success();

    let assert = common_cmd()
        .arg(COMMAND_MAX)
        .arg("0.1.2+bm0")
        .arg("0.1.2+bm1")
        .assert();
    assert
        .append_context(COMMAND_MAX, "ambiguous boundary")
        .failure();

    let assert = common_cmd()
        .arg("-o")
        .arg("text")
        .arg(COMMAND_MAX)
        .arg("--allow-ambiguous")
        .arg("0.1.2+bm0")
        .arg("0.1.2+bm1")
        .assert();
    assert
        .append_context(COMMAND_MAX, "allow ambiguous")
        .success();

    let assert = common_cmd()
        .arg("-o")
        .arg("text")
        .arg(COMMAND_MAX)
        .arg("--stable")
        .arg("1.0.0-alpha")
        .arg("1.0.0")
        .arg("2.0.0")
        .assert();
    assert
        .append_context(COMMAND_MAX, "stable max")
        .stdout("2.0.0\n")
        .success();

    let assert = common_cmd()
        .arg("-o")
        .arg("text")
        .arg(COMMAND_LATEST)
        .arg("1.0.0")
        .arg("2.0.0")
        .assert();
    assert
        .append_context(COMMAND_LATEST, "latest alias")
        .stdout("2.0.0\n")
        .success();
}

fn version_without_build(v: &Version) -> Version {
    Version {
        major: v.major,
        minor: v.minor,
        patch: v.patch,
        pre: v.pre.clone(),
        build: BuildMetadata::EMPTY,
    }
}

fn boundary_ambiguous(versions: &[Version], kind_max: bool) -> bool {
    if versions.is_empty() {
        return false;
    }
    let mut groups: HashMap<Version, usize> = HashMap::new();
    for v in versions {
        *groups.entry(version_without_build(v)).or_insert(0) += 1;
    }
    let key = if kind_max {
        groups.keys().max().cloned()
    } else {
        groups.keys().min().cloned()
    };
    key.and_then(|k| groups.get(&k).copied())
        .map(|c| c > 1)
        .unwrap_or(false)
}

fn apply_stable(versions: &[Version], stable: bool) -> Vec<Version> {
    if stable {
        versions
            .iter()
            .filter(|v| v.pre.is_empty())
            .cloned()
            .collect()
    } else {
        versions.to_vec()
    }
}

fn apply_filter(versions: &[Version], filter: &Option<VersionReq>) -> Vec<Version> {
    match filter {
        Some(f) => versions.iter().filter(|v| f.matches(v)).cloned().collect(),
        None => versions.to_vec(),
    }
}

const BOUNDARY_TEST_VERSION_COUNT_SMALL: usize = 16;

fn boundary_test_generic(
    command: &'static str,
    kind_max: bool,
    stable: bool,
    lexical_sorting: bool,
    allow_ambiguous: bool,
    filter: Option<VersionReq>,
    versions: Vec<Version>,
) {
    let filtered = apply_filter(&apply_stable(&versions, stable), &filter);
    if filtered.is_empty() {
        let mut args = vec!["-o".to_string(), "text".to_string(), command.to_string()];
        if stable {
            args.push("--stable".to_string());
        }
        if let Some(filter) = &filter {
            args.push("--filter".to_string());
            args.push(filter.to_string());
        }
        args.extend(versions.iter().map(|v| v.to_string()));
        common_cmd()
            .args(&args)
            .assert()
            .append_context(command, "empty after filters")
            .failure();
        return;
    }

    let ambiguous = boundary_ambiguous(&filtered, kind_max);

    let mut args = vec!["-o".to_string(), "text".to_string(), command.to_string()];
    if stable {
        args.push("--stable".to_string());
    }
    if lexical_sorting {
        args.push("--lexical-sorting".to_string());
    }
    if allow_ambiguous {
        args.push("--allow-ambiguous".to_string());
    }
    if let Some(filter) = filter {
        args.push("--filter".to_string());
        args.push(filter.to_string());
    }
    args.extend(versions.iter().map(|v| v.to_string()));

    let assert = common_cmd().args(&args).assert();
    if ambiguous && !allow_ambiguous && !lexical_sorting {
        assert.append_context(command, "ambiguous").failure();
    } else {
        let stdout = String::from_utf8_lossy(&assert.get_output().stdout).into_owned();
        assert.append_context(command, "success").success();
        let outputs: Vec<Version> = stdout
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| Version::parse(l).expect("parse output"))
            .collect();
        assert!(!outputs.is_empty());
        if allow_ambiguous && ambiguous {
            assert!(outputs.len() > 1);
        } else {
            assert_eq!(outputs.len(), 1);
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        fork: true,
        cases: 256,
        .. ProptestConfig::default()
    })]
    #[test]
    fn prop_max_small(
        stable: bool,
        lexical_sorting: bool,
        allow_ambiguous: bool,
        filter in arb_optional_version_req(0.5, 2),
        versions in arb_vec_versions(BOUNDARY_TEST_VERSION_COUNT_SMALL),
    ) {
        boundary_test_generic(COMMAND_MAX, true, stable, lexical_sorting, allow_ambiguous, filter, versions);
    }

    #[test]
    fn prop_min_small(
        stable: bool,
        lexical_sorting: bool,
        allow_ambiguous: bool,
        filter in arb_optional_version_req(0.5, 2),
        versions in arb_vec_versions(BOUNDARY_TEST_VERSION_COUNT_SMALL),
    ) {
        boundary_test_generic(COMMAND_MIN, false, stable, lexical_sorting, allow_ambiguous, filter, versions);
    }

    #[test]
    fn prop_latest_alias(
        versions in arb_vec_versions(8),
    ) {
        prop_assume!(!versions.is_empty());
        let max = common_cmd()
            .arg("-o")
            .arg("text")
            .arg(COMMAND_MAX)
            .args(versions.iter().map(|v| v.to_string()))
            .assert();
        let latest = common_cmd()
            .arg("-o")
            .arg("text")
            .arg(COMMAND_LATEST)
            .args(versions.iter().map(|v| v.to_string()))
            .assert();
        if max.get_output().status.success() {
            let max_stdout = String::from_utf8_lossy(&max.get_output().stdout).into_owned();
            let latest_stdout =
                String::from_utf8_lossy(&latest.get_output().stdout).into_owned();
            latest.append_context(COMMAND_LATEST, "alias").success();
            prop_assert_eq!(max_stdout, latest_stdout);
        } else {
            latest.append_context(COMMAND_LATEST, "alias fail").failure();
        }
    }
}

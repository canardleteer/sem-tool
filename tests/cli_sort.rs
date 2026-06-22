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
fn cli_sort_invalid_input() {
    let assert = common_cmd().arg(COMMAND_SORT).arg("a.b.c").assert();
    assert
        .append_context(COMMAND_SORT, "1 bad semver args")
        .failure();
}

#[test]
fn cli_sort_basic_cases() {
    let assert = common_cmd().arg(COMMAND_SORT).arg("0.1.2-rc0").assert();
    assert.append_context(COMMAND_SORT, "1 item").success();

    let assert = common_cmd()
        .arg(COMMAND_SORT)
        .arg("-f >0")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, -f >0")
        .success();

    let assert = common_cmd()
        .arg(COMMAND_SORT)
        .arg("-f >1")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, -f >1")
        .success();

    let assert = common_cmd()
        .arg(COMMAND_SORT)
        .arg("-f >a")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, -f >a")
        .failure();

    let assert = common_cmd()
        .arg(COMMAND_SORT)
        .arg("-r")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert.append_context(COMMAND_SORT, "2 items, -r").success();

    let assert = common_cmd()
        .arg(COMMAND_SORT)
        .arg("--flatten")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, --flatten")
        .success();

    let assert = common_cmd()
        .arg(COMMAND_SORT)
        .arg("--lexical-sorting")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, --lexical-sorting,")
        .success();

    let assert = common_cmd()
        .arg(COMMAND_SORT)
        .arg("--lexical-sorting")
        .arg("--flatten")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, --lexical-sorting, --flatten")
        .success();

    let assert = common_cmd()
        .arg(COMMAND_SORT)
        .arg("--fail-if-potentially-ambiguous")
        .arg("0.1.2+bm0")
        .arg("0.1.2+bm1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, --lexical-sorting, --flatten")
        .failure();

    let assert = common_cmd()
        .arg(COMMAND_SORT)
        .arg("0.1.2+bm0")
        .arg("0.1.2+bm1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, --lexical-sorting, --flatten")
        .success();

    let assert = common_cmd()
        .arg(COMMAND_SORT)
        .arg("--fail-if-potentially-ambiguous")
        .arg("--flatten")
        .arg("0.1.2+bm0")
        .arg("0.1.2+bm1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, --lexical-sorting, --flatten")
        .failure();

    let assert = common_cmd()
        .arg(COMMAND_SORT)
        .arg("--flatten")
        .arg("0.1.2+bm0")
        .arg("0.1.2+bm1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, --lexical-sorting, --flatten")
        .success();
}

const SORT_TEST_VERSION_COUNT_SMALL: usize = 32;
#[cfg(not(windows))]
const SORT_TEST_VERSION_COUNT_LARGE: usize = 128;

fn version_without_build(v: &Version) -> Version {
    Version {
        major: v.major,
        minor: v.minor,
        patch: v.patch,
        pre: v.pre.clone(),
        build: BuildMetadata::EMPTY,
    }
}

fn input_potentially_ambiguous(versions: &[Version]) -> bool {
    let mut counts: HashMap<Version, usize> = HashMap::new();
    for v in versions {
        *counts.entry(version_without_build(v)).or_insert(0) += 1;
    }
    counts.values().any(|&c| c > 1)
}

fn apply_stable_filter(versions: &[Version], stable: bool) -> Vec<Version> {
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

fn apply_sort_filter(versions: &[Version], filter: &Option<VersionReq>) -> Vec<Version> {
    match filter {
        Some(f) => versions.iter().filter(|v| f.matches(v)).cloned().collect(),
        None => versions.to_vec(),
    }
}

fn sort_test_generic(
    lexical_sorting: bool,
    reverse: bool,
    flatten: bool,
    fail_if_potentially_ambiguous: bool,
    stable: bool,
    filter: Option<VersionReq>,
    versions: Vec<Version>,
) {
    let mut args = Vec::new();
    if stable {
        args.push("-o".to_string());
        args.push("text".to_string());
    }
    args.push(COMMAND_SORT.to_string());
    if lexical_sorting {
        args.push("--lexical-sorting".to_string());
    }
    if reverse {
        args.push("--reverse".to_string());
    }
    if flatten {
        args.push("--flatten".to_string());
    }
    if fail_if_potentially_ambiguous {
        args.push("--fail-if-potentially-ambiguous".to_string());
    }
    if stable {
        args.push("--stable".to_string());
    }
    if stable && !flatten {
        args.push("--flatten".to_string());
    }

    if let Some(ref filter) = filter {
        args.push("--filter".to_string());
        args.push(format!("{}", filter));
    }

    args.append(
        &mut versions
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
    );

    let assert = common_cmd().args(&args).assert();
    let filtered = apply_sort_filter(&apply_stable_filter(&versions, stable), &filter);
    let ambiguous = input_potentially_ambiguous(&filtered);
    if fail_if_potentially_ambiguous && ambiguous {
        assert
            .append_context(COMMAND_SORT, "prop test ambiguous")
            .failure();
    } else if stable && filtered.is_empty() {
        let stdout = String::from_utf8_lossy(&assert.get_output().stdout).into_owned();
        assert.append_context(COMMAND_SORT, "prop test stable empty").success();
        assert!(stdout.lines().all(|l| l.is_empty()));
    } else if stable {
        let stdout = String::from_utf8_lossy(&assert.get_output().stdout).into_owned();
        assert.append_context(COMMAND_SORT, "prop test").success();
        for line in stdout.lines().filter(|l| !l.is_empty()) {
            let parsed = Version::parse(line).expect("stable sort output parses");
            assert!(parsed.pre.is_empty());
        }
    } else {
        assert.append_context(COMMAND_SORT, "prop test").success();
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        // Setting both fork and timeout is redundant since timeout implies
        // fork, but both are shown for clarity.
        fork: true,
        // timeout: 10000,
        cases: 256,
        .. ProptestConfig::default()
    })]
    // Using some large number of filters is unlikely to provide us with half the
    // test cases,
    #[test]
    fn sort_test_small(lexical_sorting: bool, reverse: bool, flatten: bool, fail_if_potentially_ambiguous: bool, stable: bool, filter in arb_optional_version_req(0.5, 2), versions in arb_vec_versions(SORT_TEST_VERSION_COUNT_SMALL)) {
        sort_test_generic(lexical_sorting, reverse, flatten, fail_if_potentially_ambiguous, stable, filter, versions);
    }

    // Since the filters are incredibly complex from the framework, the odds of
    // not making mutually exclusive comparators is small. We're just testing
    // huge input here.
    //
    // FIXME(canardleteer): Windows tests are disabled, becuase Windows is
    // incapable of passing "large" versions. We should detect that and
    // change the meaning of "large for windows" instead.
    #[cfg(not(windows))]
    #[test]
    fn sort_test_large(lexical_sorting: bool, reverse: bool, flatten: bool, fail_if_potentially_ambiguous: bool, stable: bool, filter in arb_optional_version_req(0.5, 2), versions in arb_vec_versions(SORT_TEST_VERSION_COUNT_LARGE)) {
        sort_test_generic(lexical_sorting, reverse, flatten, fail_if_potentially_ambiguous, stable, filter, versions);
    }
}

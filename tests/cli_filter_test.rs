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

mod common;
use common::subcommands::*;

use crate::common::common_cmd;

#[test]
fn cli_filter_invalid_input() {
    let assert = common_cmd().arg(COMMAND_FILTER_TEST).arg(">a.b.c").assert();
    assert
        .append_context(COMMAND_FILTER_TEST, "1 bad semver filter arg")
        .failure();

    let assert = common_cmd()
        .arg(COMMAND_FILTER_TEST)
        .arg(">1")
        .arg("x.y.z")
        .assert();
    assert
        .append_context(COMMAND_FILTER_TEST, "1 bad semver arg")
        .failure();

    let assert = common_cmd()
        .arg(COMMAND_FILTER_TEST)
        .arg("2.0.0")
        .arg(">1")
        .assert();
    assert
        .append_context(COMMAND_FILTER_TEST, "backwards args")
        .failure();
}

#[test]
fn cli_filter_test_basic_cases() {
    let assert = common_cmd()
        .arg(COMMAND_FILTER_TEST)
        .arg(">1")
        .arg("2.0.0")
        .assert();
    assert
        .append_context(COMMAND_FILTER_TEST, ">1 test")
        .success();

    let assert = common_cmd()
        .arg(COMMAND_FILTER_TEST)
        .arg(">1")
        .arg("0.0.1-rc1.br.0+abc")
        .assert();
    assert
        .append_context(COMMAND_FILTER_TEST, ">1 0.0.1-rc1.br.0+abc")
        .failure();
    // NOTE(canardleteer): I should probably add some more complex filters.
}

fn filter_test_generic(filter: semver::VersionReq, version: semver::Version) {
    let assert = common_cmd()
        .arg("filter-test")
        .arg(filter.to_string())
        .arg(version.to_string())
        .assert();
    let res = assert
        .append_context(COMMAND_FILTER_TEST, "property test")
        .try_success();

    // It doesn't matter what our status is really, just that it aligns with what we expect.
    match res {
        Ok(_) => {
            if !filter.matches(&version) {
                panic!("the cli succeeded, when it should have failed.");
            }
        }
        Err(_) => {
            if filter.matches(&version) {
                panic!("the cli failed, when it should have succeeded.");
            }
        }
    }
}

const FILTER_TEST_COMPARATOR_LENGTH_LARGE: usize = MAX_COMPARATORS_IN_VERSION_REQ_STRING;
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
    fn filter_test_small(filter in arb_version_req(1), version in arb_version()) {
        filter_test_generic(filter, version);
    }

    // Since the filters are incredibly complex from the framework, the odds of
    // not making mutually exclusive comparators is small. We're just testing
    // huge input here.
    #[test]
    fn filter_test_large(filter in arb_version_req(FILTER_TEST_COMPARATOR_LENGTH_LARGE), version in arb_version()) {
        filter_test_generic(filter, version);
    }
}

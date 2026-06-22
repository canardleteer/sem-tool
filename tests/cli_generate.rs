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
use semver::Version;

mod common;
use common::subcommands::*;

use crate::common::common_cmd;

#[test]
fn cli_validate_invalid_input() {
    let assert = common_cmd().arg(COMMAND_GENERATE).arg("-100").assert();
    assert
        .append_context(COMMAND_GENERATE, "1 bad integer args")
        .failure();
}

#[test]
fn cli_validate_basic_cases() {
    let assert = common_cmd().arg(COMMAND_GENERATE).arg("10").assert();
    assert
        .append_context(COMMAND_GENERATE, "1 valid semver arg")
        .success();

    let assert = common_cmd().arg(COMMAND_GENERATE).arg("10").assert();
    assert
        .append_context(COMMAND_GENERATE, "1 valid semver arg")
        .success();
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
    #[test]
    fn prop_generate_small(count in 1u16..10) {
        let assert = common_cmd()
            .arg("-o")
            .arg("text")
            .arg(COMMAND_GENERATE)
            .arg("-s")
            .arg(count.to_string())
            .assert();
        let stdout = String::from_utf8_lossy(&assert.get_output().stdout).into_owned();
        assert.append_context(COMMAND_GENERATE, "property testing -s").success();
        let lines: Vec<&str> = stdout
            .lines()
            .filter(|l| !l.is_empty())
            .collect();
        prop_assert_eq!(lines.len(), count as usize);
        for line in lines {
            prop_assert!(Version::parse(line).is_ok());
        }
    }

    #[test]
    fn prop_generate_regex(count in 1u16..10) {
        let assert = common_cmd()
            .arg("-o")
            .arg("text")
            .arg(COMMAND_GENERATE)
            .arg(count.to_string())
            .assert();
        let stdout = String::from_utf8_lossy(&assert.get_output().stdout).into_owned();
        assert.append_context(COMMAND_GENERATE, "property testing regex").success();
        let lines: Vec<&str> = stdout
            .lines()
            .filter(|l| !l.is_empty())
            .collect();
        prop_assert_eq!(lines.len(), count as usize);
        for line in lines {
            let validate = common_cmd().arg(COMMAND_VALIDATE).arg(line).assert();
            validate.success();
        }
    }
}

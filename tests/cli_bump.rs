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
fn cli_validate_invalid_input() {
    let assert = common_cmd().arg(COMMAND_BUMP).arg("a.b.c").assert();
    assert
        .append_context(COMMAND_BUMP, "1 bad semver args")
        .failure();
}

#[test]
fn cli_validate_basic_cases() {
    let assert = common_cmd()
        .arg(COMMAND_BUMP)
        .arg("1.1.1-rc.0.a.1.b+a.0.b.1")
        .arg("--bump-major=2")
        .arg("--bump-minor=3")
        .arg("--bump-patch=4")
        .assert();
    assert
        .append_context(COMMAND_BUMP, "1 valid semver arg")
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
    fn prop_validate_small(v in arb_version(), major: Option<u64>, minor: Option<u64>, patch: Option<u64>) {
        let (major_bump, major_checked) = match major {
            None => ("".to_string(), true),
            Some(m) => {
                (format!("--bump-major={}", m), v.major.checked_add(m).is_some())
            }
        };

        let (minor_bump, minor_checked) = match minor {
            None => ("".to_string(), true),
            Some(m) => {
                (format!("--bump-minor={}", m), v.minor.checked_add(m).is_some())
            }
        };

        let (patch_bump, patch_checked) = match patch {
            None => ("".to_string(), true),
            Some(p) => {
                (format!("--bump-patch={}", p), v.patch.checked_add(p).is_some())
            }
        };

        let mut assert = common_cmd();
        assert.arg(COMMAND_BUMP).arg(v.to_string());

        if major.is_some() {
            assert.arg(major_bump);
        }

        if minor.is_some() {
            assert.arg(minor_bump);
        }

        if patch.is_some() {
            assert.arg(patch_bump);
        }

        let assert = assert.assert();

        if major_checked && minor_checked && patch_checked {
            assert.append_context(COMMAND_SET, "property testing (success expected)").success();
        } else {
            assert.append_context(COMMAND_SET, "property testing (failure expected)").failure();
        }
    }
}

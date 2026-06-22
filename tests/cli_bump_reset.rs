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
use semver::Version;

mod common;
use common::subcommands::*;

use crate::common::common_cmd;

#[test]
fn cli_bump_reset_invalid_input() {
    let assert = common_cmd().arg(COMMAND_BUMP_RESET).arg("a.b.c").assert();
    assert
        .append_context(COMMAND_BUMP_RESET, "bad semver")
        .failure();
}

#[test]
fn cli_bump_reset_basic_cases() {
    let assert = common_cmd()
        .arg("-o")
        .arg("text")
        .arg(COMMAND_BUMP_RESET)
        .arg("1.2.3")
        .assert();
    assert
        .append_context(COMMAND_BUMP_RESET, "minor reset")
        .stdout("1.3.0")
        .success();

    let assert = common_cmd()
        .arg("-o")
        .arg("text")
        .arg(COMMAND_BUMP_RESET)
        .arg("1.2.3")
        .arg("--major")
        .assert();
    assert
        .append_context(COMMAND_BUMP_RESET, "major reset")
        .stdout("2.0.0")
        .success();

    let assert = common_cmd()
        .arg("-o")
        .arg("text")
        .arg(COMMAND_BUMP_RESET)
        .arg("1.2.3-rc.1+ci.42")
        .arg("--normal-version-only")
        .assert();
    assert
        .append_context(COMMAND_BUMP_RESET, "normal only")
        .stdout("1.3.0")
        .success();
}

proptest! {
    #![proptest_config(ProptestConfig {
        fork: true,
        cases: 256,
        .. ProptestConfig::default()
    })]
    #[test]
    fn prop_bump_reset(
        v in arb_version(),
        major_reset: bool,
        clear_pre: bool,
        clear_build: bool,
        normal_only: bool,
    ) {
        let overflow = if major_reset {
            v.major == u64::MAX
        } else {
            v.minor == u64::MAX
        };

        let mut cmd = common_cmd();
        cmd.arg("-o").arg("text").arg(COMMAND_BUMP_RESET).arg(v.to_string());
        if major_reset {
            cmd.arg("--major");
        }
        if clear_pre {
            cmd.arg("--clear-pre-release");
        }
        if clear_build {
            cmd.arg("--clear-build-metadata");
        }
        if normal_only {
            cmd.arg("--normal-version-only");
        }

        let assert = cmd.assert();
        if overflow {
            assert.append_context(COMMAND_BUMP_RESET, "overflow").failure();
        } else {
            let stdout_owned =
                String::from_utf8_lossy(&assert.get_output().stdout).trim().to_string();
            assert.append_context(COMMAND_BUMP_RESET, "success").success();
            let parsed = Version::parse(&stdout_owned).expect("output is valid semver");
            if major_reset {
                prop_assert_eq!(parsed.major, v.major + 1);
                prop_assert_eq!(parsed.minor, 0);
                prop_assert_eq!(parsed.patch, 0);
            } else {
                prop_assert_eq!(parsed.minor, v.minor + 1);
                prop_assert_eq!(parsed.patch, 0);
            }
            let clear_pre_effective = normal_only || clear_pre;
            let clear_build_effective = normal_only || clear_build;
            if clear_pre_effective {
                prop_assert!(parsed.pre.is_empty());
            } else if !v.pre.is_empty() {
                prop_assert_eq!(parsed.pre.as_str(), v.pre.as_str());
            }
            if clear_build_effective {
                prop_assert!(parsed.build.is_empty());
            } else if !v.build.is_empty() {
                prop_assert_eq!(parsed.build.as_str(), v.build.as_str());
            }
        }
    }
}

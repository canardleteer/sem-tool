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
use semver::{BuildMetadata, Prerelease};

use crate::common::common_cmd;

#[test]
fn cli_validate_invalid_input() {
    let assert = common_cmd().arg(COMMAND_SET).arg("a.b.c").assert();
    assert
        .append_context(COMMAND_SET, "1 bad semver args")
        .failure();
}

#[test]
fn cli_validate_basic_cases() {
    let assert = common_cmd()
        .arg(COMMAND_SET)
        .arg("1.1.1-rc.0.a.1.b+a.0.b.1")
        .arg("--set-major=2")
        .arg("--set-minor=3")
        .arg("--set-patch=4")
        .arg("--set-pre-release=a.b.c")
        .arg("--set-build-metadata=x.y.z")
        .assert();
    assert
        .append_context(COMMAND_SET, "1 valid semver arg")
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
    fn prop_validate_small(v in arb_version(), major: Option<u64>, minor: Option<u64>, patch: Option<u64>, pre_release: Option<String>, build_metadata: Option<String>) {
        let mut assert = common_cmd();
        assert.arg(COMMAND_SET).arg(v.to_string());

        let (major_set, major_checked) = match major {
            None => ("".to_string(), true),
            Some(m) => {
                (format!("--set-major={}", m), true)
            }
        };

        let (minor_set, minor_checked) = match minor {
            None => ("".to_string(), true),
            Some(m) => {
                (format!("--set-minor={}", m), true)
            }
        };

        let (patch_set, patch_checked) = match patch {
            None => ("".to_string(), true),
            Some(p) => {
                (format!("--set-patch={}", p), true)
            }
        };

        let (pre_release_set, pre_release_checked) = match pre_release.clone() {
            None => ("".to_string(), true),
            Some(p) => {
                (format!("--set-pre-release={}", p), Prerelease::new(&p).is_ok())
            }
        };

        let (build_metadata_set, build_metadata_checked) = match build_metadata.clone() {
            None => ("".to_string(), true),
            Some(p) => {
                (format!("--set-build-metadata={}", p), BuildMetadata::new(&p).is_ok())
            }
        };

        if major.is_some() {
            assert.arg(major_set);
        }

        if minor.is_some() {
            assert.arg(minor_set);
        }

        if patch.is_some() {
            assert.arg(patch_set);
        }

        if pre_release.is_some() {
            assert.arg(pre_release_set);
        }

        if build_metadata.is_some() {
            assert.arg(build_metadata_set);
        }

        let assert = assert.assert();

        if major_checked && minor_checked && patch_checked && pre_release_checked && build_metadata_checked {
            assert.append_context(COMMAND_SET, "property testing (success expected)").success();
        } else {
            assert.append_context(COMMAND_SET, "property testing (failure expected)").failure();
        }
    }
}

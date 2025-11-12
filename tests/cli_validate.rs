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
    let assert = common_cmd().arg(COMMAND_VALIDATE).arg("a.b.c").assert();
    assert
        .append_context(COMMAND_VALIDATE, "1 bad semver args")
        .failure();
}

#[test]
fn cli_validate_basic_cases() {
    let assert = common_cmd()
        .arg(COMMAND_VALIDATE)
        .arg("0.1.2-rc.0.a.1.b+a.0.b.1")
        .assert();
    assert
        .append_context(COMMAND_VALIDATE, "1 valid semver arg")
        .success();

    let assert = common_cmd().arg(COMMAND_VALIDATE).arg("0.0.0a").assert();
    assert
        .append_context(
            COMMAND_VALIDATE,
            "regression of: https://github.com/canardleteer/sem-tool/issues/50",
        )
        .failure();
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
    fn prop_validate_small(v in arb_version()) {
        let assert = common_cmd().arg(COMMAND_VALIDATE).arg("-s").arg(v.to_string()).assert();
        assert.append_context(COMMAND_VALIDATE, "property testing").success();
    }

    #[test]
    fn prop_validate_regex(v in arb_semver()) {
        let assert = common_cmd().arg(COMMAND_VALIDATE).arg(v).assert();
        assert.append_context(COMMAND_VALIDATE, "property testing").success();
    }
}

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
use assert_cmd::Command;

mod common;
use common::subcommands::*;

#[test]
fn cli_validate_invalid_input() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(COMMAND_GENERATE).arg("-100").assert();
    assert
        .append_context(COMMAND_GENERATE, "1 bad integer args")
        .failure();
}

#[test]
fn cli_validate_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(COMMAND_GENERATE).arg("10").assert();
    assert
        .append_context(COMMAND_GENERATE, "1 valid semver arg")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(COMMAND_GENERATE).arg("10").assert();
    assert
        .append_context(COMMAND_GENERATE, "1 valid semver arg")
        .success();
}

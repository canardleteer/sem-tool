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
fn cli_filter_invalid_input() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(COMMAND_FILTER_TEST).arg(">a.b.c").assert();
    assert
        .append_context(COMMAND_FILTER_TEST, "1 bad semver filter arg")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(COMMAND_FILTER_TEST).arg(">1").arg("x.y.z").assert();
    assert
        .append_context(COMMAND_FILTER_TEST, "1 bad semver arg")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(COMMAND_FILTER_TEST).arg("2.0.0").arg(">1").assert();
    assert
        .append_context(COMMAND_FILTER_TEST, "backwards args")
        .failure();
}

#[test]
fn cli_filter_test_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(COMMAND_FILTER_TEST).arg(">1").arg("2.0.0").assert();
    assert
        .append_context(COMMAND_FILTER_TEST, ">1 test")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_FILTER_TEST)
        .arg(">1")
        .arg("0.0.1-rc1.br.0+abc")
        .assert();
    assert
        .append_context(COMMAND_FILTER_TEST, ">1 0.0.1-rc1.br.0+abc")
        .failure();

    // NOTE(canardleteer): I should probably add some more complex filters.
}

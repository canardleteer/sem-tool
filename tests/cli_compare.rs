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
fn cli_compare_invalid_input() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(COMMAND_COMPARE).arg("a.b.c").assert();
    assert
        .append_context(COMMAND_COMPARE, "1 bad semver args")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(COMMAND_COMPARE).arg("a.b.c").arg("x.y.z").assert();
    assert
        .append_context(COMMAND_COMPARE, "2 bad semver args")
        .failure();
}

/// NOTE(canardleteer): Since these codes are considered unstable for now,
///                     be prepared to make changes in here.
#[test]
fn cli_compare_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(COMMAND_COMPARE).arg("1.2.3").arg("4.5.6").assert();

    assert
        .append_context(COMMAND_COMPARE, "no exit code reporting")
        .success();

    // Should be (sem: Equal, lex: Equal) aka Success
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_COMPARE)
        .arg("-e")
        .arg("1.2.3")
        .arg("1.2.3")
        .assert();
    assert
        .append_context(COMMAND_COMPARE, "exit code reporting")
        .success();

    // Should be (sem: Less, lex: Less) aka 100
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_COMPARE)
        .arg("-e")
        .arg("1.2.3")
        .arg("4.5.6")
        .assert();
    assert
        .append_context(COMMAND_COMPARE, "exit code reporting")
        .code(100);

    // Should be (sem: Greater, lex: Greater) aka 122
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_COMPARE)
        .arg("-e")
        .arg("4.5.6")
        .arg("1.2.3")
        .assert();
    assert
        .append_context(COMMAND_COMPARE, "exit code reporting")
        .code(122);

    // Should be (sem: Equal, lex: Greater) aka 112
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_COMPARE)
        .arg("-e")
        .arg("1.2.3+1")
        .arg("1.2.3+0")
        .assert();
    assert
        .append_context(COMMAND_COMPARE, "exit code reporting")
        .code(112);

    // Should be (sem: Equal, lex: Less) aka 110
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_COMPARE)
        .arg("-e")
        .arg("1.2.3+0")
        .arg("1.2.3+1")
        .assert();
    assert
        .append_context(COMMAND_COMPARE, "exit code reporting")
        .code(110);

    // Should be (sem: Equal, lex: Less) aka 110, but overridden by -s
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_COMPARE)
        .arg("-e")
        .arg("-s")
        .arg("1.2.3+0")
        .arg("1.2.3+1")
        .assert();
    assert
        .append_context(
            COMMAND_COMPARE,
            "exit code reporting + semantic equivalence passing",
        )
        .success();

    // Should be (sem: Less, lex: Less) aka 100, and where -s has no impact
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_COMPARE)
        .arg("-e")
        .arg("-s")
        .arg("1.2.2")
        .arg("1.2.3+1")
        .assert();
    assert
        .append_context(
            COMMAND_COMPARE,
            "exit code reporting + semantic equivalence passing",
        )
        .code(100);

    // These don't match, but should pass anyways since -e is not set
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_COMPARE)
        .arg("-s")
        .arg("1.2.4+0")
        .arg("1.2.3+1")
        .assert();
    assert
        .append_context(
            COMMAND_COMPARE,
            "semantic equivalence passing without complex exit code reporting",
        )
        .success();
}

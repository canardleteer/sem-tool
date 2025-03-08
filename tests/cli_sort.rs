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
fn cli_sort_invalid_input() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(COMMAND_SORT).arg("a.b.c").assert();
    assert
        .append_context(COMMAND_SORT, "1 bad semver args")
        .failure();
}

#[test]
fn cli_sort_basic_cases() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(COMMAND_SORT).arg("0.1.2-rc0").assert();
    assert.append_context(COMMAND_SORT, "1 item").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_SORT)
        .arg("-f >0")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, -f >0")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_SORT)
        .arg("-f >1")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, -f >1")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_SORT)
        .arg("-f >a")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, -f >a")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_SORT)
        .arg("-r")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert.append_context(COMMAND_SORT, "2 items, -r").success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_SORT)
        .arg("--flatten")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, --flatten")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_SORT)
        .arg("--lexical-sorting")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, --lexical-sorting,")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_SORT)
        .arg("--lexical-sorting")
        .arg("--flatten")
        .arg("0.1.2-rc0")
        .arg("0.1.2-rc1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, --lexical-sorting, --flatten")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_SORT)
        .arg("--fail-if-potentially-ambiguous")
        .arg("0.1.2+bm0")
        .arg("0.1.2+bm1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, --lexical-sorting, --flatten")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_SORT)
        .arg("0.1.2+bm0")
        .arg("0.1.2+bm1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, --lexical-sorting, --flatten")
        .success();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_SORT)
        .arg("--fail-if-potentially-ambiguous")
        .arg("--flatten")
        .arg("0.1.2+bm0")
        .arg("0.1.2+bm1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, --lexical-sorting, --flatten")
        .failure();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .arg(COMMAND_SORT)
        .arg("--flatten")
        .arg("0.1.2+bm0")
        .arg("0.1.2+bm1")
        .assert();
    assert
        .append_context(COMMAND_SORT, "2 items, --lexical-sorting, --flatten")
        .success();
}

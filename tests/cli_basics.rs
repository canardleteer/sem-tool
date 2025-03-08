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
fn cli_basics() {
    // Fail with no input.
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .assert()
        .failure();

    // Success with --help.
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("--help").assert();
    assert.append_context("basics", "help").success();
}

#[test]
fn cli_all_sub_commands() {
    for sub in ALL_COMMANDS {
        // All subcommands with no input, should fail, except those that do
        // something else reasonable.
        //
        // 'sort' & 'generate' have behaviors that reasonably allow them to
        // pass.
        match sub {
            "sort" | "generate" => {
                Command::cargo_bin(env!("CARGO_PKG_NAME"))
                    .unwrap()
                    .arg(sub)
                    .assert()
                    .success();
            }
            _ => {
                Command::cargo_bin(env!("CARGO_PKG_NAME"))
                    .unwrap()
                    .arg(sub)
                    .assert()
                    .failure();
            }
        }

        // All subcommands asking for --help, should pass.
        //
        // Exceptions may eventually apply, but not yet.
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .arg(sub)
            .arg("--help")
            .assert()
            .success();
    }
}

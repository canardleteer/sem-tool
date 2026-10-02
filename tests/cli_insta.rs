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
//!
//! Insta coverage for default CLI output.
use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use std::process::Command;

mod common;

fn cli() -> Command {
    Command::new(get_cargo_bin("sem-tool"))
}

#[test]
fn cli_insta() {
    let targets = common::cli_insta_cases::insta_targets();
    for (key, args) in &targets {
        // These snapshots pin 0.2.x quoting. The opt-in spelling is checked
        // by cli_yaml_quoting; all other CLI snapshots apply in both modes.
        if cfg!(feature = "plain-yaml-scalars")
            && matches!(
                *key,
                "bump.simple.2"
                    | "set.simple.2"
                    | "sort.complex.2"
                    | "sort.flatten.1"
                    | "sort.lexical.1"
                    | "sort.lexical.2"
                    | "sort.reverse.1"
                    | "sort.stable.1"
                    | "sort.unary.1"
            )
        {
            continue;
        }
        assert_cmd_snapshot!(*key, cli().args(args));
    }
}

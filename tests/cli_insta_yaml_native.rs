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
//! Insta coverage for native noyalib YAML output (`serde-yaml-compatability` disabled).
//!
//! Run with: `cargo test --no-default-features --test cli_insta_yaml_native`
//!
//! Temporary: remove this test crate when the compat feature is dropped in v0.2.0.
#![cfg(not(feature = "serde-yaml-compatability"))]

use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use std::process::Command;

mod common;

fn cli() -> Command {
    Command::new(get_cargo_bin("sem-tool"))
}

#[test]
fn cli_insta_yaml_native() {
    let targets = common::cli_insta_cases::insta_targets();
    for (key, args) in &targets {
        assert_cmd_snapshot!(*key, cli().args(args));
    }
}

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
fn cli_select_invalid_semver() {
    let assert = common_cmd()
        .arg(COMMAND_SELECT)
        .arg("major")
        .arg("a.b.c")
        .assert();
    assert
        .append_context(COMMAND_SELECT, "invalid semver")
        .failure();
}

#[test]
fn cli_select_basic_major() {
    let assert = common_cmd()
        .arg(COMMAND_SELECT)
        .arg("major")
        .arg("1.2.3")
        .assert();
    assert
        .append_context(COMMAND_SELECT, "major")
        .stdout("---\nvalue: \"1\"\n")
        .success();
}

#[test]
fn cli_select_basic_minor() {
    let assert = common_cmd()
        .arg(COMMAND_SELECT)
        .arg("minor")
        .arg("1.2.3")
        .assert();
    assert
        .append_context(COMMAND_SELECT, "minor")
        .stdout("---\nvalue: \"2\"\n")
        .success();
}

#[test]
fn cli_select_basic_patch() {
    let assert = common_cmd()
        .arg(COMMAND_SELECT)
        .arg("patch")
        .arg("0.1.2-rc.0.a.1.b+a.0.b.1")
        .assert();
    assert
        .append_context(COMMAND_SELECT, "patch")
        .stdout("---\nvalue: \"2\"\n")
        .success();
}

#[test]
fn cli_select_basic_pre_release() {
    let assert = common_cmd()
        .arg(COMMAND_SELECT)
        .arg("pre-release")
        .arg("0.1.2-rc.0.a.1.b+a.0.b.1")
        .assert();
    assert
        .append_context(COMMAND_SELECT, "pre-release")
        .stdout("---\nvalue: rc.0.a.1.b\n")
        .success();
}

#[test]
fn cli_select_basic_build_metadata() {
    let assert = common_cmd()
        .arg(COMMAND_SELECT)
        .arg("build-metadata")
        .arg("0.1.2-rc.0.a.1.b+a.0.b.1")
        .assert();
    assert
        .append_context(COMMAND_SELECT, "build-metadata")
        .stdout("---\nvalue: a.0.b.1\n")
        .success();
}

#[test]
fn cli_select_optional_missing_success() {
    let assert = common_cmd()
        .arg(COMMAND_SELECT)
        .arg("pre-release")
        .arg("1.0.0")
        .assert();
    assert
        .append_context(COMMAND_SELECT, "pre-release absent default")
        .stdout("---\n{}\n")
        .success();
}

#[test]
fn cli_select_optional_missing_fail_if_not_found() {
    let assert = common_cmd()
        .arg(COMMAND_SELECT)
        .arg("pre-release")
        .arg("1.0.0")
        .arg("--fail-if-not-found")
        .assert();
    assert
        .append_context(COMMAND_SELECT, "pre-release absent fail")
        .failure();
}

#[test]
fn cli_select_default_regex_large_version() {
    // Default (regex) path supports large numeric components
    let assert = common_cmd()
        .arg(COMMAND_SELECT)
        .arg("major")
        .arg("18446744073709551616.0.0")
        .assert();
    assert
        .append_context(COMMAND_SELECT, "regex large major")
        .stdout("---\nvalue: \"18446744073709551616\"\n")
        .success();
}

#[test]
fn cli_select_small_flag() {
    let assert = common_cmd()
        .arg(COMMAND_SELECT)
        .arg("-s")
        .arg("major")
        .arg("1.2.3")
        .assert();
    assert
        .append_context(COMMAND_SELECT, "small")
        .stdout("---\nvalue: \"1\"\n")
        .success();
}

#[test]
fn cli_select_text_output() {
    let assert = common_cmd()
        .arg("-o")
        .arg("text")
        .arg(COMMAND_SELECT)
        .arg("patch")
        .arg("2.0.4")
        .assert();
    assert
        .append_context(COMMAND_SELECT, "text")
        .stdout("4\n")
        .success();
}

proptest! {
    #![proptest_config(ProptestConfig {
        fork: true,
        cases: 256,
        ..ProptestConfig::default()
    })]
    #[test]
    fn prop_select_small(
        version in arb_version(),
        component in prop_oneof![
            Just("major"),
            Just("minor"),
            Just("patch"),
            Just("pre-release"),
            Just("build-metadata"),
        ]
    ) {
        let assert = common_cmd()
            .arg(COMMAND_SELECT)
            .arg("-s")
            .arg(component)
            .arg(version.to_string())
            .assert();
        assert.append_context(COMMAND_SELECT, "property test -s").success();
    }

    #[test]
    fn prop_select_regex(
        version in arb_semver(),
        component in prop_oneof![
            Just("major"),
            Just("minor"),
            Just("patch"),
            Just("pre-release"),
            Just("build-metadata"),
        ]
    ) {
        let assert = common_cmd()
            .arg(COMMAND_SELECT)
            .arg(component)
            .arg(version)
            .assert();
        assert.append_context(COMMAND_SELECT, "property test regex").success();
    }

    #[test]
    fn prop_select_fail_if_not_found_missing_optional(
        version in arb_version(),
        component in prop_oneof![Just("pre-release"), Just("build-metadata")],
    ) {
        prop_assume!(version.pre.is_empty());
        prop_assume!(version.build.is_empty());
        let assert = common_cmd()
            .arg(COMMAND_SELECT)
            .arg(component)
            .arg(version.to_string())
            .arg("--fail-if-not-found")
            .assert();
        assert.append_context(COMMAND_SELECT, "property test -F missing").failure();
    }

    #[test]
    fn prop_select_required_components(
        version in arb_version(),
        component in prop_oneof![Just("major"), Just("minor"), Just("patch")],
    ) {
        let assert = common_cmd()
            .arg("-o")
            .arg("text")
            .arg(COMMAND_SELECT)
            .arg(component)
            .arg(version.to_string())
            .assert();
        let stdout = String::from_utf8_lossy(&assert.get_output().stdout).trim().to_string();
        assert.append_context(COMMAND_SELECT, "property test required").success();
        prop_assert!(!stdout.is_empty());
    }
}

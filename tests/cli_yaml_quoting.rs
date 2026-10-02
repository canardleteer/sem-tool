//! SPDX-License-Identifier: Apache-2.0
//! Copyright 2026 canardleteer
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

mod common;

#[test]
fn yaml_version_string_quoting_follows_feature() {
    let expected = if cfg!(feature = "plain-yaml-scalars") {
        "---\nmutated_version: 1.3.0"
    } else {
        "---\nmutated_version: \"1.3.0\""
    };
    common::common_cmd()
        .args(["bump-reset", "1.2.3"])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn yaml_version_mapping_keys_follow_feature() {
    let expected = if cfg!(feature = "plain-yaml-scalars") {
        "---\nversions:\n  0.1.2-rc0:\n    - 0.1.2-rc0\npotentially_ambiguous: false"
    } else {
        "---\nversions:\n  \"0.1.2-rc0\":\n    - \"0.1.2-rc0\"\npotentially_ambiguous: false"
    };
    common::common_cmd()
        .args(["sort", "0.1.2-rc0"])
        .assert()
        .success()
        .stdout(expected);
}

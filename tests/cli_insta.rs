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
use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use std::{collections::HashMap, process::Command};

mod common;
use common::subcommands::*;

fn cli() -> Command {
    Command::new(get_cargo_bin("sem-tool"))
}

#[test]
fn cli_insta() {
    // Giant map of various tests for insta.
    let mut insta_targets = HashMap::new();

    // Filter Tests
    insta_targets.insert(
        "filter.invalid-semver.1",
        vec![COMMAND_FILTER_TEST, ">a.b.c"],
    );
    insta_targets.insert(
        "filter.invalid-semver.2",
        vec![COMMAND_FILTER_TEST, ">1", "x.y.z"],
    );
    insta_targets.insert(
        "filter.invalid-order.1",
        vec![COMMAND_FILTER_TEST, "2.0.0", ">1"],
    );
    insta_targets.insert("filter.plain.1", vec![COMMAND_FILTER_TEST, ">1", "2.0.0"]);
    insta_targets.insert(
        "filter.plain.2",
        vec![COMMAND_FILTER_TEST, ">1", "0.0.1-rc1.br.0+abc"],
    );

    // Sort Tests
    insta_targets.insert("sort.unary.1", vec![COMMAND_SORT, "0.1.2-rc0"]);
    insta_targets.insert(
        "sort.complex.1",
        vec![
            COMMAND_SORT,
            "--lexical-sorting",
            "--fail-if-potentially-ambiguous",
            "0.1.2+bm0",
            "0.1.2+bm1",
        ],
    );
    insta_targets.insert(
        "sort.complex.2",
        vec![
            COMMAND_SORT,
            "--lexical-sorting",
            "--flatten",
            "0.1.2-rc0",
            "0.1.2-rc1",
        ],
    );
    insta_targets.insert(
        "sort.reverse.1",
        vec![COMMAND_SORT, "-r", "0.1.2-rc0", "0.1.2-rc1"],
    );
    insta_targets.insert(
        "sort.flatten.1",
        vec![COMMAND_SORT, "--flatten", "0.1.2-rc0", "0.1.2-rc1"],
    );
    insta_targets.insert(
        "sort.lexical.1",
        vec![COMMAND_SORT, "--lexical-sorting", "0.1.2+bm0", "0.1.2+bm1"],
    );
    insta_targets.insert(
        "sort.lexical.2",
        vec![COMMAND_SORT, "--lexical-sorting", "0.1.2-rc0", "0.1.2-rc1"],
    );
    insta_targets.insert(
        "sort.filter.1",
        vec![COMMAND_SORT, "-f", ">1", "0.1.2-rc0", "0.1.2-rc1"],
    );
    insta_targets.insert(
        "sort.filter.2",
        vec![COMMAND_SORT, "-f", ">0", "0.1.2-rc0", "0.1.2-rc1"],
    );
    insta_targets.insert(
        "sort.filter.3",
        vec![COMMAND_SORT, "-f", ">a", "0.1.2-rc0", "0.1.2-rc1"],
    );

    // Validate Tests
    insta_targets.insert("validate.invalid-semver.1", vec![COMMAND_VALIDATE, "a.b.c"]);
    insta_targets.insert(
        "validate.valid-semver.1",
        vec![COMMAND_VALIDATE, "0.1.2-rc.0.a.1.b+a.0.b.1"],
    );
    insta_targets.insert(
        "validate.short.1",
        vec![COMMAND_VALIDATE, "-s", "18446744073709551616.0.0"],
    );
    insta_targets.insert(
        "validate.short.2",
        vec![COMMAND_VALIDATE, "18446744073709551616.0.0"],
    );
    insta_targets.insert(
        "validate.regression-of-bad-regex.1",
        vec![COMMAND_VALIDATE, "-s", "0.0.0a"],
    );
    insta_targets.insert(
        "validate.regression-of-bad-regex.2",
        vec![COMMAND_VALIDATE, "0.0.0a"],
    );

    // Explain Tests
    insta_targets.insert("explain.invalid-semver.1", vec![COMMAND_EXPLAIN, "a.b.c"]);
    insta_targets.insert(
        "explain.valid-semver.1",
        vec![COMMAND_EXPLAIN, "0.1.2-rc.0.a.1.b+a.0.b.1"],
    );
    insta_targets.insert("explain.valid-semver.2", vec![COMMAND_EXPLAIN, "0.1.2"]);
    insta_targets.insert(
        "explain.valid-semver.3",
        vec![COMMAND_EXPLAIN, "0.1.2-rc.0.a.1.b"],
    );
    insta_targets.insert(
        "explain.valid-semver.4",
        vec![COMMAND_EXPLAIN, "0.1.2+a.0.b.1"],
    );

    // Compare Tests
    insta_targets.insert(
        "compare.valid-semver.1",
        vec![COMMAND_COMPARE, "1.2.3", "4.5.6"],
    );
    insta_targets.insert(
        "compare.exit-status.1",
        vec![COMMAND_COMPARE, "-e", "1.2.3", "1.2.3"],
    );
    insta_targets.insert(
        "compare.exit-status.2",
        vec![COMMAND_COMPARE, "-e", "1.2.3", "4.5.6"],
    );
    insta_targets.insert(
        "compare.exit-status.3",
        vec![COMMAND_COMPARE, "-e", "4.5.6", "1.2.3"],
    );
    insta_targets.insert(
        "compare.exit-status.4",
        vec![COMMAND_COMPARE, "-e", "1.2.3+1", "1.2.3+0"],
    );
    insta_targets.insert(
        "compare.exit-status.5",
        vec![COMMAND_COMPARE, "-e", "1.2.3+0", "1.2.3+1"],
    );
    insta_targets.insert(
        "compare.exit-status.6",
        vec![COMMAND_COMPARE, "-e", "-s", "1.2.3+0", "1.2.3+1"],
    );
    insta_targets.insert(
        "compare.complex.1",
        vec![COMMAND_COMPARE, "-e", "-s", "1.2.2", "1.2.3+1"],
    );
    insta_targets.insert(
        "compare.semantic-exit-status.1",
        vec![COMMAND_COMPARE, "-s", "1.2.4+0", "1.2.3+1"],
    );
    insta_targets.insert(
        "bump.simple.1",
        vec!["-o", "text", COMMAND_BUMP, "1.1.1", "--bump-major=1"],
    );
    insta_targets.insert(
        "bump.simple.2",
        vec![COMMAND_BUMP, "1.1.1", "--bump-major=1"],
    );
    insta_targets.insert(
        "set.simple.1",
        vec!["-o", "text", COMMAND_SET, "1.1.1", "--set-major=20"],
    );
    insta_targets.insert("set.simple.2", vec![COMMAND_SET, "1.1.1", "--set-major=20"]);

    for (key, args) in insta_targets.iter() {
        assert_cmd_snapshot!(*key, cli().args(args));
    }
}

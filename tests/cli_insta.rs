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

/// When set (e.g. in CI for the mcp feature job), skip the default insta test so we don't
/// compare against default-feature snapshots. The mcp binary has different --help (e.g. --mcp,
/// --export-skills) and possibly other divergence.
fn skip_default_insta() -> bool {
    std::env::var("SEM_TOOL_SKIP_INSTA").unwrap_or_default() == "1"
}

#[test]
fn cli_insta() {
    if skip_default_insta() {
        return;
    }
    // Giant map of various tests for insta.
    let mut insta_targets = HashMap::new();

    // --help for root and each subcommand (snapshot CLI help text)
    insta_targets.insert("help.root", vec!["--help"]);
    insta_targets.insert("help.explain", vec![COMMAND_EXPLAIN, "--help"]);
    insta_targets.insert("help.compare", vec![COMMAND_COMPARE, "--help"]);
    insta_targets.insert("help.sort", vec![COMMAND_SORT, "--help"]);
    insta_targets.insert("help.filter-test", vec![COMMAND_FILTER_TEST, "--help"]);
    insta_targets.insert("help.validate", vec![COMMAND_VALIDATE, "--help"]);
    insta_targets.insert("help.generate", vec![COMMAND_GENERATE, "--help"]);
    insta_targets.insert("help.set", vec![COMMAND_SET, "--help"]);
    insta_targets.insert("help.bump", vec![COMMAND_BUMP, "--help"]);

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
        insta::with_settings!({
            filters => vec![
                (r"sem-tool\.exe", "sem-tool"),
            ]
        }, {
            assert_cmd_snapshot!(*key, cli().args(args));
        });
    }
}

/// Snapshots --help output for the MCP-enabled binary. Run with `SEM_TOOL_MCP_HELP=1` when
/// testing the `mcp` feature build so we have separate snapshots (mcp adds --mcp, --export-skills,
/// etc.). In CI the mcp job sets both SEM_TOOL_SKIP_INSTA and SEM_TOOL_MCP_HELP.
#[test]
fn cli_insta_mcp_help() {
    if std::env::var("SEM_TOOL_MCP_HELP").unwrap_or_default() != "1" {
        return;
    }
    let help_cases = [
        ("help_mcp.root", vec!["--help"]),
        ("help_mcp.explain", vec![COMMAND_EXPLAIN, "--help"]),
        ("help_mcp.compare", vec![COMMAND_COMPARE, "--help"]),
        ("help_mcp.sort", vec![COMMAND_SORT, "--help"]),
        ("help_mcp.filter-test", vec![COMMAND_FILTER_TEST, "--help"]),
        ("help_mcp.validate", vec![COMMAND_VALIDATE, "--help"]),
        ("help_mcp.generate", vec![COMMAND_GENERATE, "--help"]),
        ("help_mcp.set", vec![COMMAND_SET, "--help"]),
        ("help_mcp.bump", vec![COMMAND_BUMP, "--help"]),
    ];
    for (name, args) in help_cases {
        insta::with_settings!({
            filters => vec![
                (r"sem-tool\.exe", "sem-tool"),
            ]
        }, {
            assert_cmd_snapshot!(name, cli().args(args));
        });
    }
}

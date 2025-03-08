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
use std::process::Command;

mod common;
use common::subcommands::*;

fn cli() -> Command {
    Command::new(get_cargo_bin("sem-tool"))
}

#[test]
fn cli_insta() {
    // NOTE(canardleteer): This should be a map with snapshot names.
    //
    //                     Until then, if these get reordered, after
    //                     confirming correctness, you'll need to use
    //                     `INSTA_UPDATE=always` locally to reset snapshots.
    //
    // NOTE(canardleteer): No `generate` tests yet.
    let insta_targets = vec![
        vec![COMMAND_FILTER_TEST, ">a.b.c"],
        vec![COMMAND_FILTER_TEST, ">1", "x.y.z"],
        vec![COMMAND_FILTER_TEST, "2.0.0", ">1"],
        vec![COMMAND_FILTER_TEST, ">1", "2.0.0"],
        vec![COMMAND_FILTER_TEST, ">1", "0.0.1-rc1.br.0+abc"],
        vec![COMMAND_SORT, "0.1.2-rc0"],
        vec![
            COMMAND_SORT,
            "--lexical-sorting",
            "--fail-if-potentially-ambiguous",
            "0.1.2+bm0",
            "0.1.2+bm1",
        ],
        vec![COMMAND_SORT, "--lexical-sorting", "0.1.2+bm0", "0.1.2+bm1"],
        vec![COMMAND_SORT, "-r", "0.1.2-rc0", "0.1.2-rc1"],
        vec![COMMAND_SORT, "--flatten", "0.1.2-rc0", "0.1.2-rc1"],
        vec![COMMAND_SORT, "--lexical-sorting", "0.1.2-rc0", "0.1.2-rc1"],
        vec![
            COMMAND_SORT,
            "--lexical-sorting",
            "--flatten",
            "0.1.2-rc0",
            "0.1.2-rc1",
        ],
        vec![COMMAND_SORT, "-f", ">1", "0.1.2-rc0", "0.1.2-rc1"],
        vec![COMMAND_SORT, "-f", ">0", "0.1.2-rc0", "0.1.2-rc1"],
        vec![COMMAND_SORT, "-f", ">a", "0.1.2-rc0", "0.1.2-rc1"],
        vec![COMMAND_VALIDATE, "a.b.c"],
        vec![COMMAND_VALIDATE, "0.1.2-rc.0.a.1.b+a.0.b.1"],
        vec![COMMAND_VALIDATE, "-s", "18446744073709551616.0.0"],
        vec![COMMAND_VALIDATE, "18446744073709551616.0.0"],
        vec![COMMAND_EXPLAIN, "a.b.c"],
        vec![COMMAND_EXPLAIN, "0.1.2-rc.0.a.1.b+a.0.b.1"],
        vec![COMMAND_COMPARE, "1.2.3", "4.5.6"],
        vec![COMMAND_COMPARE, "-e", "1.2.3", "1.2.3"],
        vec![COMMAND_COMPARE, "-e", "1.2.3", "4.5.6"],
        vec![COMMAND_COMPARE, "-e", "4.5.6", "1.2.3"],
        vec![COMMAND_COMPARE, "-e", "1.2.3+1", "1.2.3+0"],
        vec![COMMAND_COMPARE, "-e", "1.2.3+0", "1.2.3+1"],
        vec![COMMAND_COMPARE, "-e", "-s", "1.2.3+0", "1.2.3+1"],
        vec![COMMAND_COMPARE, "-e", "-s", "1.2.2", "1.2.3+1"],
        vec![COMMAND_COMPARE, "-s", "1.2.4+0", "1.2.3+1"],
        // NOTE(canardleteer): For now, the `generate` command is omitted.
    ];

    for args in insta_targets {
        assert_cmd_snapshot!(cli().args(args));
    }
}

use assert_cmd::{Command, cargo::cargo_bin_cmd};

#[allow(dead_code)]
pub(crate) mod subcommands {
    pub(crate) const COMMAND_COMPARE: &str = "compare";
    pub(crate) const COMMAND_EXPLAIN: &str = "explain";
    pub(crate) const COMMAND_FILTER_TEST: &str = "filter-test";
    pub(crate) const COMMAND_GENERATE: &str = "generate";
    pub(crate) const COMMAND_SORT: &str = "sort";
    pub(crate) const COMMAND_VALIDATE: &str = "validate";
    pub(crate) const COMMAND_SET: &str = "set";
    pub(crate) const COMMAND_BUMP: &str = "bump";
    pub(crate) const COMMAND_SELECT: &str = "select";
    pub(crate) const ALL_COMMANDS: [&str; 9] = [
        COMMAND_COMPARE,
        COMMAND_EXPLAIN,
        COMMAND_FILTER_TEST,
        COMMAND_GENERATE,
        COMMAND_SORT,
        COMMAND_VALIDATE,
        COMMAND_BUMP,
        COMMAND_SET,
        COMMAND_SELECT,
    ];
}

#[allow(dead_code)]
pub(crate) fn common_cmd() -> Command {
    cargo_bin_cmd!(env!("CARGO_PKG_NAME"))
}

#[allow(dead_code)]
pub(crate) fn compare_cmd(a: &str, b: &str) -> Command {
    let mut cmd = common_cmd();
    cmd.arg(subcommands::COMMAND_COMPARE)
        .arg("--a")
        .arg(a)
        .arg("--b")
        .arg(b);
    cmd
}

#[allow(dead_code)]
pub(crate) fn filter_test_cmd(filter: &str, semantic_version: &str) -> Command {
    let mut cmd = common_cmd();
    cmd.arg(subcommands::COMMAND_FILTER_TEST)
        .arg("--filter")
        .arg(filter)
        .arg("--semantic-version")
        .arg(semantic_version);
    cmd
}

#[allow(dead_code)]
pub(crate) fn select_cmd(component: &str, version: &str) -> Command {
    let mut cmd = common_cmd();
    cmd.arg(subcommands::COMMAND_SELECT)
        .arg("--component")
        .arg(component)
        .arg("--version")
        .arg(version);
    cmd
}

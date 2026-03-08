//! SPDX-License-Identifier: Apache-2.0
//! Copyright 2025 canardleteer
//!
//! Binary entrypoint for sem-tool-mcp; delegates to sem-tool with MCP feature.

fn main() -> std::process::ExitCode {
    match sem_tool::run_app() {
        Ok(exit_code) => exit_code,
        Err(e) => {
            eprintln!("{e}");
            std::process::ExitCode::FAILURE
        }
    }
}

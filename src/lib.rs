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

//! `sem-tool` is a CLI application. This crate root exists so integration tests
//! can share a small amount of logic with the implementation; it is not a
//! supported library API.

mod cli;
mod misc;
mod regex;
mod results;

use std::error::Error;
use std::process::{ExitCode, Termination};

/// Entry point for the `sem-tool` binary.
#[doc(hidden)]
pub fn run() -> Result<ExitCode, Box<dyn Error>> {
    cli::run().map(|outcome| outcome.report())
}

/// Hidden exports for integration tests only.
#[doc(hidden)]
pub mod test_support {
    pub use crate::results::{
        compare_exit_code, ordering_pair_to_exit_code, version_without_build_metadata,
    };
}

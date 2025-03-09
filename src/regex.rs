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
use std::string::FromUtf8Error;

use rand::{Rng, distr::Distribution};
use semver::{BuildMetadata, Prerelease};

/// Regex for Semantic Version 2.0.0, directly from the spec, with 2 changes:
///
/// * ASCII Only Restriction
/// * No prepended `^` or trailing `$`, since [proptest!] uses this with the
///   [regex_generate](https://github.com/CryptArchy/regex_generate) crate.
pub const SEMVER_REGEX: &str = r"(?-u:(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?)";

/// Regex to build a Pre-Release string, always, without the `-`.
pub const ALWAYS_PRERELEASE_REGEX: &str = r"(?-u:(?:((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*)))";

/// Regex to build a Build Metadata string, always, without the prefix `+`.
pub const ALWAYS_BUILD_METADATA_REGEX: &str = r"(?-u:(?:([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*)))";

/// Generally the "maximum limit on repetitions" allowed for the regex string
/// generator, to prevent automata making overly useless decisions for the
/// purposes of this tool.
///
/// See [rand_regex::Regex] for more information.
pub(crate) const DEFAULT_MAX_REPEAT: u32 = 100;

/// Generate [Vec<String>] filled with valid Semantic Versions.
pub(crate) fn generate_any_valid_semver(count: usize) -> Vec<String> {
    let mut rng = rand::rng();
    let semver = rand_regex::Regex::compile(SEMVER_REGEX, DEFAULT_MAX_REPEAT).unwrap();

    (&mut rng)
        .sample_iter(&semver)
        .take(count)
        .collect::<Vec<String>>()
}

/// Generate [Vec<String>] filled with valid Semantic Versions bound by [u64::MAX]
/// promises for MAJOR, MINOR and PATCH.
///
/// This could probably be done better.
pub(crate) fn generate_u64_safe_semver(count: usize) -> Vec<String> {
    type RandomStringRegexIter = rand::distr::Iter<
        rand_regex::Regex,
        rand::prelude::ThreadRng,
        Result<std::string::String, FromUtf8Error>,
    >;

    let mut rng = rand::rng();

    let pre_release_gen: RandomStringRegexIter =
        rand_regex::Regex::compile(ALWAYS_PRERELEASE_REGEX, DEFAULT_MAX_REPEAT)
            .unwrap()
            .sample_iter(rand::rng());
    let build_metadata_gen: RandomStringRegexIter =
        rand_regex::Regex::compile(ALWAYS_BUILD_METADATA_REGEX, DEFAULT_MAX_REPEAT)
            .unwrap()
            .sample_iter(rand::rng());

    // Because our regexes exclude UTF-8 and are "to form", we feel confident in
    // unwrapping here.
    pre_release_gen
        .zip(build_metadata_gen)
        .map(|(pr, bm)| {
            format!(
                "{}.{}.{}{}{}",
                rng.random::<u64>(),
                rng.random::<u64>(),
                rng.random::<u64>(),
                match rng.random_bool(0.5) {
                    true => format!("-{}", Prerelease::new(&pr.unwrap()).unwrap()),
                    _ => "".to_string(),
                },
                match rng.random_bool(0.5) {
                    true => format!("+{}", BuildMetadata::new(&bm.unwrap()).unwrap()),
                    _ => "".to_string(),
                }
            )
        })
        .take(count)
        .collect::<Vec<String>>()
}

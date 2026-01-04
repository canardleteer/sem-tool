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
//! This source file doesn't contain much more than just the basics of
//! CLI documentation, and routing to the appropriate place.

//! NOTE(canardleteer): We allow bare_urls, because CLI documentation is
//!                     more important than rust-doc here.
#![allow(rustdoc::bare_urls)]

use clap::{Parser, Subcommand};
use semver::{Version, VersionReq};
use std::error::Error;
use std::io;

mod misc;
mod regex;
mod results;

use misc::*;
use results::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    cmd: Commands,

    #[clap(long, short = 'o', value_enum, default_value_t=OutputFormat::Yaml)]
    out: OutputFormat,
}

/// All commands available
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Explain a valid Semantic Version as parsed by the spec.
    ///
    /// Breaks apart the Semantic Version, into it's individual components.
    ///
    /// All values are returned as strings, because the unsigned integer
    /// types are not necessarily bound by a numeric type that is parsable
    /// by common libraries.
    ///
    /// It is worth noting, Semver 2.0.0 §11.4.1 & §11.4.2 pre-release &
    /// metadata dot separated values, cannot be negative numbers, since
    /// they cannot be represented with hyphens.
    ///
    /// Reference: https://semver.org/#spec-item-11
    ///
    Explain { semantic_version: Version },
    /// Compare 2 Semantic Versions.
    ///
    /// Results are provided in the form
    /// "A is {Greater,Equals,Less} {to,than} B", with both Semantic results
    /// (meaningful results under Semantic Versioning), as well as Lexical
    /// results (meaningless, but handy for sorting text lists).
    ///
    /// Without enabling `--set_exit_status`, the exit status is generally
    /// meaningless, other than confirming that the arguments were valid.
    Compare {
        /// If you want some slightly complex exit status codes for this dual
        /// compare, you can turn them on with this flag.
        ///
        /// When both Semantic and Lexical comparisons are Equal, the command
        /// will end with an exit status of 0 (Success).
        ///
        /// All other outcomes, are returned with an exit status of the form: 1XY [between 100-122].
        ///
        ///   - With X being (0 if Less, 1 if Equal, 2 if Greater) on the Semantic Compare
        ///
        ///   - With Y being (0 if Less, 1 if Equal, 2 if Greater) on the Lexical Compare
        ///
        /// The non-0 exit status codes, should be considered UNSTABLE, because something
        /// better can probably be figured out.
        #[clap(long, short = 'e', action)]
        set_exit_status: bool,
        /// Always exit with success when Semantic Versions are Equal.
        ///
        /// Mostly impacts the output when the flag `set_exit_status` is set.
        #[clap(long, short = 's', action)]
        semantic_exit_status: bool,
        /// The base version used for comparison.
        a: Version,
        /// The version we are comparing against.
        b: Version,
    },
    /// Sort a list of valid Semantic Versions, with either Semantic or Lexical ordering.
    ///
    /// Results are grouped by default, under the meaningful components of Semantic
    /// Versioning (without build metadata), then enumerated under that component.
    Sort {
        #[clap(long, short = 'f', default_value = None)]
        /// Only emit versions that match a filter.
        ///
        /// These filter rules are described by the semver crate `VersionReq``
        /// documentation, and more generally in the cargo book.
        ///
        /// In particular, note the warnings around pre-releases in the
        /// VersionReq documentation.
        ///
        /// References:
        /// - https://docs.rs/semver/1.0.25/semver/struct.VersionReq.html
        /// - https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
        filter: Option<VersionReq>,

        #[clap(long, action)]
        /// Lexical Sorting (aka Total Order).
        ///
        /// WARNING: This may lead to bad choices surrounding semantic
        /// versioning,
        ///
        /// This is bound to be controversial, but worth understanding.
        ///
        /// Semver 2.0.0 §10 states that:
        /// "Build metadata MUST be ignored when determining version
        /// precedence."
        ///
        /// This has been set to the default behavior of emulating undefined
        /// behavior, because it MUST be ignored. It is quite common, for
        /// people to accidentally choose the sorting order of their favorite
        /// or most familiar tool, and not the specification itself. This
        /// enforces by default, the ignoring of the version precedence.
        ///
        /// Additionally, we must interpret the following statement as
        /// undefined ordering for the case where Build Metadata may be `None`
        /// or `Some`:
        ///
        /// "Thus two versions that differ only in the build metadata, have
        /// the same precedence."
        ///
        /// References:
        /// - https://semver.org/#spec-item-10
        lexical_sorting: bool,

        #[clap(long, short = 'r', action)]
        /// Reverses ordering.
        ///
        /// Note, "reversing" always effects the comparable versions being
        /// ordered, but is ignored when NOT lexically sorted, for the list of
        /// semantically identical versions (aka, different metadata). Since by
        /// default they are randomly sorted, there is no point.
        reverse: bool,

        #[clap(long, action)]
        /// Flatten the map, and provide a list of versions.
        ///
        /// WARNING: This may lead to bad choices surrounding semantic
        /// versioning.
        flatten: bool,

        #[clap(long, action)]
        /// Fail, if potentially ambiguous precedence may emerge from these
        /// versions (multiple matching M.M.P-PR, but non-matching metadata).
        fail_if_potentially_ambiguous: bool,

        /// If no versions are present, then the tool will read from stdin, one
        /// version per line.
        versions: Option<Vec<Version>>,
    },
    /// Test a Semantic Version against a filter
    FilterTest {
        /// Filter to test against a specific Semantic Version.
        ///
        /// These filter rules are described by the semver crate `VersionReq``
        /// documentation, and more generally in the cargo book.
        ///
        /// In particular, note the warnings around pre-releases in the
        /// VersionReq documentation.
        ///
        /// The Status Code will be 0 if it passes, non-zero if it fails.
        ///
        /// References:
        /// - https://docs.rs/semver/1.0.25/semver/struct.VersionReq.html
        /// - https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
        filter: VersionReq,

        /// Version to test
        semantic_version: Version,
    },
    /// Simply validates an argument, to confirm it is a valid Semantic Version
    ///
    /// The Status Code will be 0 if it is valid, non-zero if it is not.
    Validate {
        /// Version to validate
        version: String,

        /// "Small" will ensure the MAJOR, MINOR & PATCH components are under [u64::MAX].
        #[clap(long, short = 's', action)]
        small: bool,
    },
    /// Generate random & valid Semantic Version Strings
    Generate {
        /// "Small" will ensure the MAJOR, MINOR & PATCH components are under [u64::MAX].
        #[clap(long, short = 's', action)]
        small: bool,

        /// How many to create (default 1)
        #[clap(default_value_t = 1)]
        count: usize,
    },
    /// Set commands will replace one segment of semver, with a specified one,
    /// and print out the mutated version.
    Set {
        semantic_version: Version,

        #[clap(long, action)]
        set_major: Option<u64>,
        #[clap(long, action)]
        set_minor: Option<u64>,
        #[clap(long, action)]
        set_patch: Option<u64>,
        #[clap(long, action)]
        set_pre_release: Option<String>,
        #[clap(long, action)]
        set_build_metadata: Option<String>,
    },
    /// Bump commands will increment one segment of semver by the specified
    /// amount, and print out the mutated version.
    ///
    /// Only Major, Minor and Patch are supported. You'll want to consider the
    /// `set` subcommand for pre-release and build-metadata.
    Bump {
        semantic_version: Version,

        #[clap(long, action)]
        bump_major: Option<u64>,
        #[clap(long, action)]
        bump_minor: Option<u64>,
        #[clap(long, action)]
        bump_patch: Option<u64>,
        // NOTE(canardleteer): We could actually do bumps on pre-release and
        //                     build-metadata, if we supported a selector and
        //                     confirmed the segment chosen was numeric.
    },
}

fn main() -> Result<ApplicationTermination, Box<dyn Error>> {
    let args = Args::parse();

    let mut ignore_exit_status_from_output = false;

    let application_output: ApplicationOutput = match args.cmd {
        Commands::Explain { semantic_version } => explain(&semantic_version).into(),
        Commands::Compare {
            set_exit_status,
            semantic_exit_status,
            a,
            b,
        } => {
            // If we don't consider non-equivalence an error, don't report one
            // on process exit.
            if !set_exit_status {
                ignore_exit_status_from_output = true;
            }
            let res = compare(&a, &b);

            if semantic_exit_status && res.semantic_ordering() == &SerializableOrdering::Equal {
                ignore_exit_status_from_output = true
            }

            res.into()
        }
        Commands::Sort {
            versions,
            filter,
            lexical_sorting,
            reverse,
            flatten,
            fail_if_potentially_ambiguous,
        } => {
            let mut parsed_versions = Vec::new();

            // Read from stdin, or pass forward the pre-parsed list from the arguments
            match versions {
                Some(versions) => parsed_versions = versions,
                None => {
                    let lines = io::stdin().lines();
                    for (line_no, line) in lines.enumerate() {
                        match line {
                            Ok(line) => {
                                let line = line.trim();
                                parsed_versions.push(Version::parse(line)
                                .map_err(|e| {
                                    eprintln!("unable to parse an enumerated version: line {line_no}: {line}: {e}");
                                    e
                                })?);
                                Ok(())
                            }
                            Err(e) => {
                                eprintln!("unable to read from stdin: {e}");
                                Err(ApplicationError::InvalidArgument {
                                    expected: "to be able to read from stdin".to_string(),
                                    found: e.to_string(),
                                })
                            }
                        }?
                    }
                }
            }

            let mut ordered_version_list =
                sort(&mut parsed_versions, &filter, lexical_sorting, reverse);

            if fail_if_potentially_ambiguous && ordered_version_list.potentially_ambiguous() {
                return Err(Box::new(misc::ApplicationError::FailedRequirementError {
                    err: "Potential Ambiguity Detected".to_string(),
                }));
            }

            match flatten {
                true => FlatVersionsList::from(&mut ordered_version_list).into(),
                false => ordered_version_list.into(),
            }
        }
        Commands::FilterTest {
            filter,
            semantic_version,
        } => filter_test(&filter, &semantic_version).into(),
        Commands::Validate { version, small } => validate(version, small).into(),
        Commands::Generate { small, count } => generate(small, count).into(),
        Commands::Set {
            semantic_version,
            set_major,
            set_minor,
            set_patch,
            set_pre_release,
            set_build_metadata,
        } => set(
            &semantic_version,
            set_major,
            set_minor,
            set_patch,
            set_pre_release,
            set_build_metadata,
        )?
        .into(),
        Commands::Bump {
            semantic_version,
            bump_major,
            bump_minor,
            bump_patch,
        } => bump(&semantic_version, bump_major, bump_minor, bump_patch)?.into(),
    };

    match args.out {
        OutputFormat::Text => print!("{application_output}"),
        OutputFormat::Yaml => {
            println!("---");
            let yaml = serde_yaml::to_string(&application_output)
                .map_err(|e| ApplicationError::OutputFormatError { err: e.to_string() })?;
            print!("{yaml}");
        }
        OutputFormat::Json => {
            let json = serde_json::to_string(&application_output)
                .map_err(|e| ApplicationError::OutputFormatError { err: e.to_string() })?;
            println!("{json}");
        }
    }

    Ok(ApplicationTermination::new(
        application_output,
        ignore_exit_status_from_output,
    ))
}

fn sort(
    versions: &mut Vec<Version>,
    filter: &Option<VersionReq>,
    lexical_sorting: bool,
    reverse: bool,
) -> OrderedVersionMap {
    OrderedVersionMap::new(versions, filter, lexical_sorting, reverse)
}

/// Returns the semantic and lexical equivalence of 2 versions.
fn compare(a: &Version, b: &Version) -> ComparisonStatement {
    ComparisonStatement::new(a, b)
}

fn explain(v: &Version) -> VersionExplanation {
    VersionExplanation::from(v)
}

fn filter_test(filter: &VersionReq, semantic_version: &Version) -> FilterTestResult {
    FilterTestResult::filter_test(filter, semantic_version)
}

fn validate(semantic_version: String, small: bool) -> ValidateResult {
    // NOTE(canardleteer): This is somewhat of a useless code path.
    ValidateResult::validate(semantic_version, small)
}

fn generate(small: bool, count: usize) -> GenerateResult {
    GenerateResult::new(small, count)
}

fn set(
    version: &Version,
    major: Option<u64>,
    minor: Option<u64>,
    patch: Option<u64>,
    pre_release: Option<String>,
    build_metadata: Option<String>,
) -> Result<VersionMutationResult, Box<dyn Error>> {
    VersionMutationResult::set(version, major, minor, patch, pre_release, build_metadata)
}

fn bump(
    version: &Version,
    major: Option<u64>,
    minor: Option<u64>,
    patch: Option<u64>,
) -> Result<VersionMutationResult, Box<dyn Error>> {
    VersionMutationResult::bump(version, major, minor, patch)
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use proptest_semver::*;

    use crate::{SerializableOrdering, version_without_build_metadata};

    proptest! {
        //                 None of these tests do much more than ensure the
        //                 application doesn't bounce back or crash on valid
        //                 input.
        #[test]
        fn compare(a in arb_version(), b in arb_version()) {
            let comparison = super::compare(&a, &b);

            let a_no_build = version_without_build_metadata(&a);
            let b_no_build = version_without_build_metadata(&b);

            match a.cmp(&b) {
                std::cmp::Ordering::Equal => {
                    prop_assert_eq!(a_no_build, b_no_build);
                    prop_assert!(comparison.semantic_ordering() == &SerializableOrdering::Equal);
                },
                std::cmp::Ordering::Greater => {
                    prop_assert!(comparison.lexical_ordering() == &SerializableOrdering::Greater || comparison.lexical_ordering() == &SerializableOrdering::Equal);
                    prop_assert_eq!(comparison.semantic_ordering(), &SerializableOrdering::Greater);
                },
                std::cmp::Ordering::Less => {
                    prop_assert!(comparison.lexical_ordering() == &SerializableOrdering::Less || comparison.lexical_ordering() == &SerializableOrdering::Equal);
                    prop_assert_eq!(comparison.semantic_ordering(), &SerializableOrdering::Less);
                },

            };
        }

        #[test]
        fn explain(version in arb_version()) {
            super::explain(&version);
        }

        #[test]
        fn validate(version in arb_semver(), small: bool) {
            super::validate(version, small);
        }

        #[test]
        fn filter_test(filter in arb_version_req(MAX_COMPARATORS_IN_VERSION_REQ_STRING), version in arb_version()) {
            super::filter_test(&filter, &version);
        }

        #[test]
        fn sort(versions in arb_vec_versions(256), filter in arb_optional_version_req(0.5, MAX_COMPARATORS_IN_VERSION_REQ_STRING), lexical_sorting in any::<bool>(), reverse in any::<bool>()) {
            let mut versions = versions.clone();
            super::sort(&mut versions, &filter, lexical_sorting, reverse);
        }

        #[test]
        fn generate(small: bool, count: u8) {
            // Not going to flex maxing out memory allocations here, limiting
            // to u8 testing.
            super::generate(small, count.into());
        }
    }
}

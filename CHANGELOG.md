# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.9](https://github.com/canardleteer/sem-tool/compare/v0.1.8...v0.1.9) - 2025-03-11

### Other

- use Self where possible
- misc typos
- automigrate to rust2024
- performance on PreMetaSegment ascii check

## [0.1.8](https://github.com/canardleteer/sem-tool/compare/v0.1.7...v0.1.8) - 2025-03-09

### Added

- replace regex_generate with rand_regex
- reduce binary size

### Fixed

- don't print empty optional data in explain
- misc typos

### Other

- remove stray comment
- update insta tests
- sort ordered versions exactly once
- reduce magic subcommands
- typos
- reduce cli test scaffolding + clippy
- note on where to further reduce dependency size
- remove magic command name from CLI tests
- license pass
- update dependencies

## [0.1.7](https://github.com/canardleteer/sem-tool/compare/v0.1.6...v0.1.7) - 2025-03-07

### Added

- add --fail-if-potentially-ambiguous flag to sort
- allow for regex validation or semver crate validation

## [0.1.6](https://github.com/canardleteer/sem-tool/compare/v0.1.5...v0.1.6) - 2025-03-06

### Added

- sem-tool generate
- simple cli snapshotting

### Other

- add note about older rand in Cargo.toml
- sem-tool generate
- clean up cli tests (again)
- fixup README some more
- limitations + release page

## [0.1.5-rc1.4](https://github.com/canardleteer/sem-tool/compare/v0.1.3...v0.1.5-rc1.4) - 2025-02-24

### Added

- add validate command

### Fixed

- enrich the cli_sort tests
- fixup various cli tests
- better testing on cli_basics

### Other

- validate unit test
- update README with testing updates
- known limitations

## [0.1.3](https://github.com/canardleteer/sem-tool/compare/v0.1.2...v0.1.3) - 2025-02-23

### Added

- lightweight explain and sort cli tests
- add compare --semantic-exit-status

### Other

- clean up compare exit code docs (slightly)

## [0.1.2](https://github.com/canardleteer/sem-tool/compare/v0.1.1...v0.1.2) - 2025-02-23

### Added

- add some lightweight cli tests
- cli application exit codes
- add building as part of testing

### Other

- update README
- add obligatory badges

## [0.1.1](https://github.com/canardleteer/sem-tool/compare/v0.1.0...v0.1.1) - 2025-02-16

### Other

- clean up CLI docs

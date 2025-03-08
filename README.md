# sem-tool

> **A simple tool for working with [Semantic Versioning](https://semver.org/) on the command line.**

[![Crates.io](https://img.shields.io/crates/v/sem-tool?style=flat-square)](https://crates.io/crates/sem-tool)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/canardleteer/sem-tool/testing.yml?branch=main&style=flat-square)](https://github.com/canardleteer/sem-tool/actions/workflows/testing.yml?query=branch%3Amain)

Semantic Versioning seems simple, but in many cases, it's not implemented
correctly, and people only consider the MAJOR.MINOR.PATCH cases. When this
happens, you may find yourself needing to do surgery on lists of versions
in a pipeline or even just to reason around someones releases.

The Rust ecosystem, as well as most Cloud Native patterns have adopted
SemVer. Not everyone has, and sometimes a little grease is needed to get
systems back on track where there's divergence.

It can be damaging to a project, when Semantic Versioning is only
partially or incorrectly implemented. It's a foundational communication
mechanism for engineers, and should be treated with the care and diligence
it's owed.

This is a single tool to provide support for that purpose.

## Opinions

Where appropriate opinions on the spec have been made, they have been
listed in the CLI documentation.

In particular, we use the `semver` crate's interpretation of "filters,"
for matching. This is **NOT** in the specification, so subject to
interpretation.

## Known Limitations

- We (currently) use the [semver crate](https://crates.io/crates/semver), which
  has some limitations not present in the spec.
  - `u64::MAX` is sometimes the maximum a value can be in any of `MAJOR`,
    `MINOR` or `PATCH`.
  - The maximum number of comparators for a filter, is 32.
- In all cases where a Regular Expression is used, we only accept ASCII input.

## Installing

- The easy way, if you use Rust, is:

```shell
cargo install sem-tool
```

- [Releases](https://github.com/canardleteer/sem-tool/releases) have various installer
  patterns for multiple Operating Systems and Package Management tools.

## Running

Your best place to start, is:

```shell
sem-tool --help
```

## Output

Currently, the following output types are: `yaml`, `text`, `json`.

I favor the `yaml` output, and have made that the default.

Exit Status is available either by default in obvious cases, or by flag in less obvious cases.

## Subcommands

### `filter-test`

The `filter-test` subcommand will allow you to test a filter on a version.

- Versions, must have `MAJOR`, `MINOR`, `PATCH` components under `u64::MAX`.

```shell
# Passing test
$ sem-tool filter-test ">=1.0.3" 1.0.3
---
pass: true
$ echo $?
0

# Failing test
$ sem-tool filter-test ">=1.0.3" 1.0.1
---
pass: false
$ echo $?
1
```

### `validate`

The `validate` subcommand just helps a script determine if a string is a valid Semantic Version or not.

```shell
# Passing test
$ sem-tool validate 1.0.3-x+m
---
valid: true
$ echo $?
0

# Failing test
$ sem-tool validate a.b.c
---
valid: false
$ echo $?
1

# Passing test (major = u64::MAX+1)
$ sem-tool validate 18446744073709551616.0.0
---
valid: true
$ echo $?
0

# Failing test (major = u64::MAX+1)
$ sem-tool validate -s 18446744073709551616.0.0
---
valid: false
$ echo $?
1
```

### `explain`

The `explain` subcommand will break down a version by components.

The `sem-tool explain --help` command has some useful
information regarding "why" the output may appear "over-stringified"
in the breakdown.

- Versions, must have `MAJOR`, `MINOR`, `PATCH` components under `u64::MAX`.

```shell
$ sem-tool explain 10.1.4-a.b.c+sda.4
---
major: 10
minor: 1
patch: 4
prerelease_string: a.b.c
prerelease:
- kind: Ascii
  value: a
- kind: Ascii
  value: b
- kind: Ascii
  value: c
build_metadata_string: sda.4
build-metadata:
- kind: Ascii
  value: sda
- kind: Numeric
  value: '4'
```

### `compare`

- Versions, must have `MAJOR`, `MINOR`, `PATCH` components under `u64::MAX`.

```shell
# simple case
$ sem-tool compare 1.2.3 2.2.2
---
semantic_ordering: Less
lexical_ordering: Less
$ echo $?
0

# simple case with status code reporting enabled
$ sem-tool compare -e 1.2.3 2.2.2
---
semantic_ordering: Less
lexical_ordering: Less
$ echo $?
100

# comparing 2 "equal" versions
$ sem-tool compare 2.2.2+abc 2.2.2
---
semantic_ordering: Equal
lexical_ordering: Greater
$ echo $?
0

# comparing 2 "equal" versions with status code reporting enabled
$ sem-tool compare -e 2.2.2+abc 2.2.2
---
semantic_ordering: Equal
lexical_ordering: Greater
$ echo $?
112

$ sem-tool compare -es 2.2.2+abc 2.2.2
---
semantic_ordering: Equal
lexical_ordering: Greater
$ echo $?
0
```

### `sort`

The `sort` command is somewhat complex, but offers 2 different modes of input:

- CLI arguments
- reading from standard input

It is recommended that you read `sem-tool sort --help`, but here are some
examples. If you're wondering why you may sometimes get different results
than these, it's once again, helpful to read the `--help`.

The result additionally includes a flag called `potentially_ambiguous`, which
can be used to identify potentially ambiguous Semantic Versions (any "order" is
valid).

- Versions, must have `MAJOR`, `MINOR`, `PATCH` components under `u64::MAX`.

#### `sort` with CLI arguments

- Versions, must have `MAJOR`, `MINOR`, `PATCH` components under `u64::MAX`.

```shell
# simple cli argument sorting
$ sem-tool sort 1.2.3 3.2.1 2.2.2
---
versions:
  1.2.3:
  - 1.2.3
  2.2.2:
  - 2.2.2
  3.2.1:
  - 3.2.1
potentially_ambiguous: false

# simple cli argument sorting, reverse ordering
$ sem-tool sort -r 1.2.3 3.2.1 2.2.2
---
versions:
  3.2.1:
  - 3.2.1
  2.2.2:
  - 2.2.2
  1.2.3:
  - 1.2.3
potentially_ambiguous: false

# filtering
$ sem-tool sort -f ">=2" -r 1.2.3 3.2.1 2.2.2
---
versions:
  3.2.1:
  - 3.2.1
  2.2.2:
  - 2.2.2
potentially_ambiguous: false

# check for potential ambiguity
$ sem-tool sort --fail-if-potentially-ambiguous 1.2.3+bm0 2.2.0 2.2.0+bm0
Error: FailedRequirementError { err: "Potential Ambiguity Detected" }
```

#### `sort` with standard input

```shell
# stdin argument sorting
$ cat example-data/short-good-versions.txt | sem-tool sort
---
versions:
  0.0.0-alpha.0:
  - 0.0.0-alpha.0+metadata
  0.0.1:
  - 0.0.1
  0.0.2:
  - 0.0.2
  0.2.0:
  - 0.2.0
  1.0.0-rc-2:
  - 1.0.0-rc-2+aaaaaa
  1.0.0-rc-2.0:
  - 1.0.0-rc-2.0+aaa.0
  - 1.0.0-rc-2.0+dddddd
  99.99.0-rc1.0:
  - 99.99.0-rc1.0
potentially_ambiguous: true

# reverse ordering
$ cat example-data/short-good-versions.txt | sem-tool sort -r
---
versions:
  99.99.0-rc1.0:
  - 99.99.0-rc1.0
  1.0.0-rc-2.0:
  - 1.0.0-rc-2.0+aaa.0
  - 1.0.0-rc-2.0+dddddd
  1.0.0-rc-2:
  - 1.0.0-rc-2+aaaaaa
  0.2.0:
  - 0.2.0
  0.0.2:
  - 0.0.2
  0.0.1:
  - 0.0.1
  0.0.0-alpha.0:
  - 0.0.0-alpha.0+metadata
potentially_ambiguous: true

# filtering (see --help regarding how this filter applies)
$ cat example-data/short-good-versions.txt | sem-tool sort -r -f '*'
---
versions:
  0.2.0:
  - 0.2.0
  0.0.2:
  - 0.0.2
  0.0.1:
  - 0.0.1
potentially_ambiguous: false

# flattening (not recommended)
$ cat example-data/short-good-versions.txt | sem-tool sort --flatten
---
versions:
- 0.0.0-alpha.0+metadata
- 0.0.1
- 0.0.2
- 0.2.0
- 1.0.0-rc-2+aaaaaa
- 1.0.0-rc-2.0+aaa.0
- 1.0.0-rc-2.0+dddddd
- 99.99.0-rc1.0
potentially_ambiguous: true

# flat list of latest matching a filter as a plain list
$ cat example-data/short-good-versions.txt | sem-tool  -o text sort --flatten -r -f "*" 
0.2.0
0.0.2
0.0.1
```

### `generate`

Simple "generator" of random SemVer valid strings.

I personally prefer the "text" output of these.

```shell
$ sem-tool -o text generate 2
# ... 2 potentially very long strings .... 

$ sem-tool -o text generate -s 2
# ... 2 potentially very long strings, but with MAJOR.MINOR.PATCH components
# all under u64::MAX .... 

# Dogfooding example.
$ sem-tool -o text generate -s 1000 | sem-tool sort
```

## Todo

- [ ] Possibly remove "text" output, or just make it really nice.
- [ ] Additional language filter implementations
  - [ ] This is somewhat of a slippery slope.
  - [ ] Consider if we should seek to use pure regex filtering.
- [ ] Commands that take stdin, should probably take file inputs too.
- [ ] CLI Testing (probably) with `assert_cmd`
  - [X] all subcommands
  - [ ] make these far more robust
  - [ ] restructure or add a framework to make them more legible & composable
  - [ ] validate all output types
  - [ ] output snapshotting
    - [ ] basic
    - [ ] output snapshotting of all types
    - [ ] output snapshotting with good labels
- [ ] Property testing
  - [X] There is work on [this
    branch](https://github.com/canardleteer/sem-tool/tree/proptest), but it
    needs a reorg after the CLI Testing lands in a more composable form.
- [ ] Unit Testing
  - [ ] Validate output
- [X] Need status code responses options
- [X] Potential Ambiguity
- [X] Regex Validate
- [X] Generate random semantic version lists for helping build tests
- [X] Github Actions + release-plz
  
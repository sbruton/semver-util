# semver-util

[![Crate](https://img.shields.io/crates/v/semver-util.svg)](https://crates.io/crates/semver-util)
[![Docs](https://docs.rs/semver-util/badge.svg)](https://docs.rs/semver-util)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.60+-lightgray.svg)](https://github.com/rust-random/rand#rust-version-requirements)
[![Open issues](https://img.shields.io/github/issues/sbruton/semver-util)](https://github.com/sbruton/semver-util/issues)

## Getting Started

```shell
$ cargo install semver-util
$ semver --help
Usage: semver <COMMAND>

Commands:
  cmp      Compare ordinality of two versions
  seq      Generate a sequence of versions
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

## Comparing Versions

Use the `cmp` command to check ordinality of two semantic versions. The command will output `true` or `false` to `stdout`. The process exit code is also set to `1` for commands that output `false`.

```bash
$ semver cmp 1.2.3 gt 1.2.0
true

$ semver cmp 1.2.3 lt 1.2.0
false

$ semver cmp 1.2.3 gte 1.2.3
true

$ semver cmp 1.2.3 lte 1.2.2
false

$ semver cmp 1.2.3 eq 1.2.3
true
```

**Comparing versions in another shell script**

```bash
#!/usr/bin/env bash

set -e

old_version () {
  echo "old version detected"
}

new_version () {
  echo "new version detected"
}

semver cmp $1 gt $2 > /dev/null \
  && new_version \
  || old_version
```

## Generate Version Sequences

**Minor Version Sequence**

```bash
$ semver seq --minor --minor-max 8 1.0.0 2.5.0
1.0.0
1.1.0
1.2.0
1.3.0
1.4.0
1.5.0
1.6.0
1.7.0
1.8.0
2.0.0
2.1.0
2.2.0
2.3.0
2.4.0
2.5.0
```

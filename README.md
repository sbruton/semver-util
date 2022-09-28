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
  compare
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

## Comparing Versions

Use the `compare` command to check ordinality of two semantic versions. The command will output `true` or `false` to `stdout`. The process exit code is also set to `1` for commands that output `false`.

```bash
$ semver compare 1.2.3 gt 1.2.0
true

$ semver compare 1.2.3 lt 1.2.0
false

$ semver compare 1.2.3 gte 1.2.3
true

$ semver compare 1.2.3 lte 1.2.2
false

$ semver compare 1.2.3 eq 1.2.3
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

semver compare $1 gt $2 > /dev/null \
  && new_version \
  || old_version
```

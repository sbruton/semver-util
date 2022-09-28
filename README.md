# semver-util

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

```shell
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

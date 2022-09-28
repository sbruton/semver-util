//! # Semantic Version Command Line Utility
//!
//! [![Minimum rustc version](https://img.shields.io/badge/rustc-1.60+-lightgray.svg)](https://github.com/rust-random/rand#rust-version-requirements)
//! [![Open issues](https://img.shields.io/github/issues/sbruton/semver-util)](https://github.com/sbruton/semver-util/issues)
//!
//! ## Installation
//!
//! __Install from `crates.io` Registry__
//! ```bash
//! $ cargo install semver-util
//! ```
//!
//! ## Comparing Versions
//!
//! Use the `compare` command to check ordinality of two semantic versions.
//!
//! Command will output `true` or `false` to `stdout`.
//!
//! Process exit code is also set to `1` for commands that output `false`.
//!
//! ```bash
//! $ semver compare 1.2.3 gt 1.2.0
//! true
//!
//! $ semver compare 1.2.3 lt 1.2.0
//! false
//!
//! $ semver compare 1.2.3 gte 1.2.3
//! true
//!
//! $ semver compare 1.2.3 lte 1.2.2
//! false
//!
//! $ semver compare 1.2.3 eq 1.2.3
//! true
//! ```
//! **Comparing versions in another shell script**
//!
//! ```bash
//! #!/usr/bin/env bash
//!
//! set -e
//!
//! old_version () {
//!   echo "old version detected"
//! }
//!
//! new_version () {
//!   echo "new version detected"
//! }
//!
//! semver compare $1 gt $2 > /dev/null \
//!   && new_version \
//!   || old_version
//! ```

use clap::{Parser, Subcommand};
use semver_pub::Version;
use strum::{Display, EnumString};

#[derive(Clone, Copy, Display, EnumString, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
enum Comparator {
    #[strum(serialize = "eq")]
    Equal,
    #[strum(serialize = "gt")]
    GreaterThan,
    #[strum(serialize = "gte")]
    GreaterThanOrEqual,
    #[strum(serialize = "lt")]
    LessThan,
    #[strum(serialize = "lte")]
    LessThanOrEqual,
}

#[derive(Parser)]
struct CompareArgs {
    version1: Version,
    comparator: Comparator,
    version2: Version,
}

#[derive(Subcommand)]
enum Command {
    Compare(CompareArgs),
}

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Command::Compare(compare_args) => compare(&args, compare_args),
    }
}

fn compare(_args: &Args, compare_args: &CompareArgs) {
    let result = match compare_args.comparator {
        Comparator::Equal => compare_args.version1 == compare_args.version2,
        Comparator::GreaterThan => compare_args.version1 > compare_args.version2,
        Comparator::GreaterThanOrEqual => compare_args.version1 >= compare_args.version2,
        Comparator::LessThan => compare_args.version1 < compare_args.version2,
        Comparator::LessThanOrEqual => compare_args.version1 <= compare_args.version2,
    };
    if result {
        println!("true");
        std::process::exit(0);
    } else {
        println!("false");
        std::process::exit(1);
    }
}

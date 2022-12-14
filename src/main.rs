//! # Semantic Version Command Line Utility
//!
//! [![Minimum rustc version](https://img.shields.io/badge/rustc-1.64+-lightgray.svg)](https://github.com/rust-random/rand#rust-version-requirements)
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
//! Use the `cmp` command to check ordinality of two semantic versions.
//!
//! Command will output `true` or `false` to `stdout`.
//!
//! Process exit code is also set to `1` for commands that output `false`.
//!
//! ```bash
//! $ semver cmp 1.2.3 gt 1.2.0
//! true
//!
//! $ semver cmp 1.2.3 lt 1.2.0
//! false
//!
//! $ semver cmp 1.2.3 gte 1.2.3
//! true
//!
//! $ semver cmp 1.2.3 lte 1.2.2
//! false
//!
//! $ semver cmp 1.2.3 eq 1.2.3
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
//! semver cmp $1 gt $2 > /dev/null \
//!   && new_version \
//!   || old_version
//! ```
//! ## Generate Version Sequences
//!
//! **Minor Version Sequence**
//!
//! ```bash
//! $ semver seq --minor --minor-max 8 1.0.0 2.5.0
//! 1.0.0
//! 1.1.0
//! 1.2.0
//! 1.3.0
//! 1.4.0
//! 1.5.0
//! 1.6.0
//! 1.7.0
//! 1.8.0
//! 2.0.0
//! 2.1.0
//! 2.2.0
//! 2.3.0
//! 2.4.0
//! 2.5.0
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

#[derive(Parser)]
struct SeqArgs {
    #[clap(long, default_value_t = false)]
    minor: bool,
    #[clap(long, default_value_t = 0)]
    minor_max: u64,
    #[clap(long, default_value_t = 0)]
    patch_max: u64,
    from: Version,
    to: Version,
}

#[derive(Subcommand)]
enum Command {
    /// Compare ordinality of two versions
    Cmp(CompareArgs),
    /// Generate a sequence of versions
    Seq(SeqArgs),
}

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Command::Cmp(compare_args) => cmp(&args, compare_args),
        Command::Seq(seq_args) => seq(&args, seq_args),
    }
}

fn cmp(_args: &Args, compare_args: &CompareArgs) {
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

fn seq(_args: &Args, seq_args: &SeqArgs) {
    let mut ver = seq_args.from.clone();
    loop {
        if ver <= seq_args.to {
            println!("{}", ver);
        }
        if ver >= seq_args.to {
            break;
        }
        if ver < seq_args.to {
            if seq_args.minor {
                if seq_args.minor_max <= ver.minor {
                    ver.minor = 0;
                    ver.major += 1;
                } else {
                    ver.minor += 1;
                }
                ver.patch = 0;
            } else if seq_args.patch_max <= ver.patch {
                ver.patch = 0;
                if seq_args.minor_max <= ver.minor {
                    ver.minor = 0;
                    ver.major += 1;
                } else {
                    ver.minor += 1;
                }
            } else {
                ver.patch += 1;
            }
        }
    }
}

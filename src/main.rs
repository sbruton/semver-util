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

use std::process;

use anyhow::Result;
use clap::{builder::PossibleValue, Parser, ValueEnum};
use regex::Regex;

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(default_value = ".", num_args = 1.., help = "Search paths")]
    paths: Vec<String>,
    #[arg(short = 'n', long = "name", num_args = 1.., value_name = "NAME", help = "Name")]
    names: Vec<Regex>,
    #[arg(short = 't', long = "type", num_args = 1..3, value_name = "TYPE", help = "Entry type")]
    entry_types: Vec<EntryType>,
}

fn run(args: Args) -> Result<()> {
    println!("{:?}", args);
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprint!("{e}");
        process::exit(1);
    }
}

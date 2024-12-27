use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Result;
use clap::{Arg, ArgAction, Command, Parser};

#[derive(Debug, Parser)]
struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,

    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn get_args() -> Args {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("chan")
        .about("Rust version of `cat`")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("number")
                .long("number")
                .conflicts_with("number_nonblank")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Number lines"),
        )
        .arg(
            Arg::new("number_nonblank")
                .long("number-nonblank")
                .short('b')
                .action(ArgAction::SetTrue)
                .help("Number non-blank"),
        )
        .get_matches();

    Args {
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(reader) => {
                let mut num = 1;

                for line in reader.lines() {
                    let line = line?;

                    if args.number_nonblank_lines && line.is_empty() {
                        println!();
                        continue;
                    }

                    if args.number_lines || args.number_nonblank_lines {
                        print!("{num:>6}\t");
                    }

                    println!("{}", line);

                    num += 1;
                }
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run(get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

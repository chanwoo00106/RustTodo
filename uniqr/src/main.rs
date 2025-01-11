use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    process,
};

use anyhow::Result;
use clap::{ArgAction, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value = "-", value_name = "FILE", help = "Input file name")]
    in_file: String,
    #[arg(value_name = "OUT_FILE", help = "Output file name")]
    out_file: Option<String>,
    #[arg(short = 'c', long,help = "Show counts", action = ArgAction::SetTrue)]
    count: bool,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> {
    let file = match open(&args.in_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    let mut a: Vec<String> = Vec::new();

    for line in file.lines() {
        let line = line?;
        a.push(line);
    }

    a.dedup();

    for (count, line) in a.iter().enumerate() {
        println!("{line}",);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

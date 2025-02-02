use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    process,
};

use anyhow::{anyhow, Result};
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
    let mut line = String::new();
    let mut prev_line = String::new();
    let mut count = 0;
    let mut file = open(&args.in_file).map_err(|e| anyhow!("{}: {e}", args.in_file))?;

    let mut outfile: Box<dyn Write> = match args.out_file {
        Some(out_file) => Box::new(File::create(out_file)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: i32, line: &str| -> Result<()> {
        if args.count {
            write!(&mut outfile, "{:>4} {}", count, line)?;
        } else {
            write!(&mut outfile, "{}", line)?;
        }

        Ok(())
    };

    loop {
        file.read_line(&mut line)?;

        if line.bytes().count() == 0 {
            break;
        }

        if line.trim_end() != prev_line.trim_end() {
            if count > 0 {
                print(count, &prev_line)?;
            }
            prev_line = line.clone();
            count = 0;
        }

        count += 1;

        line.clear();
    }

    if count > 0 {
        print(count, &prev_line)?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

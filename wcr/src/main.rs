use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(num_args = 1.., default_value = "-", help = "Input file(s)", value_name = "FILE")]
    files: Vec<String>,

    #[arg(help = "print the newline counts", short = 'l', long)]
    lines: bool,

    #[arg(help = "print the word counts", short = 'w', long)]
    words: bool,

    #[arg(help = "print the byte counts", short = 'c', long)]
    bytes: bool,

    #[arg(
        help = "print the character counts",
        short = 'm',
        long,
        conflicts_with = "bytes"
    )]
    chars: bool,
}

#[derive(Debug)]
struct FileInfo {
    lines_count: usize,
    words_count: usize,
    bytes_count: usize,
    chars_count: usize,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "_" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read_file_count(mut file: Box<dyn BufRead>) -> Result<FileInfo> {
    let mut lines_count = 0;
    let mut words_count = 0;
    let mut bytes_count = 0;
    let mut chars_count = 0;
    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;

        if bytes == 0 {
            break;
        }

        lines_count += 1;
        bytes_count += bytes;
        words_count += line.split_whitespace().count();
        chars_count += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        lines_count,
        words_count,
        bytes_count,
        chars_count,
    })
}

fn run(mut args: Args) -> Result<()> {
    if [args.chars, args.bytes, args.words, args.lines]
        .iter()
        .all(|a| a == &false)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("{e}"),
            Ok(file) => {
                let file_info = read_file_count(file)?;

                if args.lines {
                    print!("{:>8}", file_info.lines_count);
                }
                if args.words {
                    print!("{:>8}", file_info.words_count);
                }
                if args.bytes {
                    print!("{:>8}", file_info.bytes_count);
                }
                if args.chars {
                    print!("{:>8}", file_info.chars_count);
                }
                println!(" {filename}");
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}

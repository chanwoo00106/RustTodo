use anyhow::Result;
use clap::{value_parser, Arg, Command};
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    vec,
};

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    lines: u64,
    bytes: Option<u64>,
}

fn get_args() -> Args {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("chan")
        .about("Rust version of head")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .short('n')
                .value_parser(value_parser!(u64).range(1..))
                .default_value("10")
                .long("lines")
                .help("Number of lines [default: 10]"),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .short('c')
                .conflicts_with("lines")
                .value_parser(value_parser!(u64).range(1..))
                .default_value(Option::None)
                .long("bytes")
                .help("Number of bytes"),
        )
        .get_matches();

    Args {
        files: matches.get_many("files").unwrap().cloned().collect(),
        lines: matches.get_one("lines").cloned().unwrap(),
        bytes: matches.get_one("bytes").cloned(),
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> {
    let len = args.files.len();

    for (count, filename) in args.files.iter().enumerate() {
        if count != 0 {
            println!()
        }

        if len > 1 {
            println!("==> {} <==", filename);
        }

        match open(&filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut txt) => {
                if let Some(bytes) = args.bytes {
                    let mut buffer = vec![0; bytes as usize];
                    let bytes_read = txt.read(&mut buffer)?;

                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));

                    continue;
                }

                for _ in 0..args.lines {
                    let mut line = String::new();
                    let bytes = txt.read_line(&mut line)?;
                    if bytes == 0 {
                        break;
                    }
                    print!("{}", line);
                }
            }
        }
    }

    Ok(())
}

fn main() {
    let args = get_args();

    let _ = run(args);
}

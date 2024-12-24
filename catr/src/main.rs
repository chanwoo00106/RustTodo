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

fn main() {
    let args = Args::parse();
    println!("{args:#?}")
}

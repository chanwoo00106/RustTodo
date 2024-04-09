use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("Not found file");

    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("Something went wrong reading the file");

    println!("{}", content);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

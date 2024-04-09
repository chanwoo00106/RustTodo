use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    let mut f = File::open(filename).expect("Not found file");

    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("Something went wrong reading the file");

    println!("{}", content);
}

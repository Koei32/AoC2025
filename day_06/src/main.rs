use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    let f = File::open("./src/example.txt").expect("error opening input file");
    let reader = BufReader::new(f);
    let input: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    dbg!(input);
}

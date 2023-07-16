use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum CustomError {}

fn main() {
    let file = File::open("Cargo.toml").unwrap();
    let file_reader = BufReader::new(file);
    file_reader
        .lines()
        .for_each(|line| println!("{}\n", line.unwrap()))
}

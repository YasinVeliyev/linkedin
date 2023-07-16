use std::fs;
use std::io::Error;

fn main() {
    println!("{}", read_file("Cargo.toml").expect("File does not exists"));
}

fn read_file(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}

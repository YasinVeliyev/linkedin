use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    print_wanted_lines_from_file("Cargo.toml", "a");
}

fn print_wanted_lines_from_file(path: &str, string: &str) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().for_each(|line| {
        if line.as_ref().unwrap().contains(string) {
            println!("{}", line.unwrap());
        }
    })
}

use std::fs;
use std::io::prelude::*;

fn main() {
    let wanted_string = "a";
    let contents = fs::read_to_string("Cargo.toml").expect("Unable to open the file");
    contents.lines().for_each(|line| {
        if line.contains(wanted_string) {
            println!("{}", line)
        }
    })
}

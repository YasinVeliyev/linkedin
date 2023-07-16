use std::fs;

fn main() {
    println!("{}", read_file("Cargo.toml"));
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Unable to open the file")
}

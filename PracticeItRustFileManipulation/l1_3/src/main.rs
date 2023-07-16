use std::fs;

fn main() {
    println!(
        "{}",
        fs::read_to_string("Cargo.toml").expect("Unable to read the file")
    );
}

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = File::create("test.txt")
        .unwrap()
        .write("This is my new file".as_bytes())
        .unwrap();
    println!("Hello, world!");
}

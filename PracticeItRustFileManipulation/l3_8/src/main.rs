use std::{
    fs,
    fs::File,
    io::{Error, Write},
};

fn main() {
    let file_path = "words_to_file";
    let words = vec!["Words", "of", "the", "first", "line"];
    write_to_file(file_path, words);
    println!("Hello, world!");
}

fn write_to_file(path: &str, words: Vec<&str>) -> Result<(), Error> {
    // File::create(path)
    //     .unwrap()
    //     .write_all(words.join(" ")[..].as_bytes())
    //     .unwrap();
    fs::write(path, words.join(" "));
    Ok(())
}

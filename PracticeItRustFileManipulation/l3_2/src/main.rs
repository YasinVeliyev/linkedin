use std::{fs, io::Error};
fn main() {
    println!("{:#?}", read_file("Cargo.toml").unwrap());
}

fn read_file(path: &str) -> Result<Vec<String>, Error> {
    // Ok(fs::read_to_string(path)
    //     .into_iter()
    //     .collect::<Vec<String>>())

    Ok(fs::read_to_string(path)
        .unwrap()
        .lines()
        .into_iter()
        .map(|line| line.to_owned())
        .collect::<Vec<String>>())
}

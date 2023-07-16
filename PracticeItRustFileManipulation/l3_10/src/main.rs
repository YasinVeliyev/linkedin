use std::io::Error;

fn main() {
    println!("Hello, world!");
    println!("{:?}", read_file("file_with_lines").unwrap())
}

fn read_file(path: &str) -> Result<Vec<Vec<String>>, Error> {
    Ok(std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.to_owned())
                .collect()
        })
        .collect())
}

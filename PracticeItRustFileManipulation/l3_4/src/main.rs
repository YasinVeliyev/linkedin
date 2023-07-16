use std::io::Error;

fn main() {
    let contents = String::from("This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five");
    println!("{:#?}", get_words(&contents));
}

fn get_words(string: &str) -> Result<Vec<String>, Error> {
    Ok(string
        .split_whitespace()
        .map(|word| word.to_owned())
        .collect::<Vec<String>>())
}

use std::collections::HashMap;
use std::fs;
use std::io::Error;

fn main() {
    let replacement_map = HashMap::from([
        ("herself".to_string(), "himself".to_string()),
        ("herself,".to_string(), "himself,".to_string()),
        ("her.".to_string(), "him.".to_string()),
        ("she".to_string(), "he".to_string()),
        ("(she".to_string(), "(he".to_string()),
        ("her".to_string(), "his".to_string()),
        ("Alice's".to_string(), "Marcus's".to_string()),
        ("Alice!".to_string(), "Marcus!".to_string()),
        ("Alice,".to_string(), "Marcus,".to_string()),
        ("Alice;".to_string(), "Marcus;".to_string()),
        ("She".to_string(), "He".to_string()),
        ("(Alice".to_string(), "(Marcus".to_string()),
        ("Alice,)".to_string(), "Marcus,)".to_string()),
        ("she'll".to_string(), "he'll".to_string()),
        ("she’ll".to_string(), "he’ll".to_string()),
        ("Alice".to_string(), "Marcus".to_string()),
        ("her,".to_string(), "him,".to_string()),
        ("Alice’s".to_string(), "Marcus’s".to_string()),
        ("girl".to_string(), "boy".to_string()),
    ]);
    let mut contents = read_file("alice_chapter_1");
    let _ = contents
        .iter()
        .map(|words| replace_x_with_y(words.clone(), &replacement_map));

    write_file("marcus_chapter_1.txt", contents).unwrap();
}

fn read_file(path: &str) -> Vec<Vec<String>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|word| format!("{}", word.trim()))
                .collect()
        })
        .collect()
}

fn replace_x_with_y(contents: Vec<String>, replacement_map: &HashMap<String, String>) {
    let mut contents = contents;
    contents.iter_mut().for_each(|word| {
        if let Some(value) = replacement_map.get(word) {
            *word = value.clone()
        }
    });
}

fn write_file(path: &str, contents: Vec<Vec<String>>) -> Result<(), Error> {
    let contents = contents
        .iter()
        .map(|words| words.join(" "))
        .collect::<Vec<String>>();
    fs::write(path, contents.join("\n"))
}

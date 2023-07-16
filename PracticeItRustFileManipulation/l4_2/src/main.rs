use std::collections::HashMap;

fn main() {
    let contents = 
        "This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five";
    println!("Hello, world!");
    println!("{:#?}", count_words(contents));
}

fn count_words(contents: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    contents.split_whitespace().for_each(|word| {
        map.entry(word.trim().to_lowercase())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    });
    map
}

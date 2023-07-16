use std::collections::HashMap;

fn main() {
    let contents = String::from("This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five");
    let words = contents
        .split_whitespace()
        .map(|word| word.to_owned())
        .collect();
    let replacement_map = HashMap::from([
        ("first".to_string(), "last".to_string()),
        ("line".to_string(), "entry".to_string()),
    ]);

    println!("Hello, world!");
    println!("{:?}", replace_x_with_y(words, &replacement_map));
}

fn replace_x_with_y(
    contents: Vec<String>,
    replacement_map: &HashMap<String, String>,
) -> Vec<String> {
    let mut contents = contents;
    // contents
    //     .iter_mut()
    //     .for_each(|word| match replacement_map.get(word) {
    //         Some(value) => *word = value.clone(),
    //         None => {}
    //     });

    contents.iter_mut().for_each(|word| {
        if let Some(value) = replacement_map.get(word) {
            *word = value.clone()
        }
    });
    contents
}

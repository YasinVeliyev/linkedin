use tasks::print_any_text::print_;

#[test]
fn print_str() {
    let input = "Rust";
    print_(&input);
}

#[test]
fn print_string() {
    let input = String::from("Rust");
    print_(&input);
}

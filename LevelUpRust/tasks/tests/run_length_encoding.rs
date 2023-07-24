use tasks::run_length_encoding::RunLengthEncoding;

#[test]
fn abc() {
    assert_eq!(RunLengthEncoding::encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    let input = "LinkedIn";
    println!("{}", RunLengthEncoding::encode(input));
    assert_eq!(
        RunLengthEncoding::decode(&RunLengthEncoding::encode(input)),
        input
    );
}

#[test]
fn long_run() {
    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(RunLengthEncoding::encode(input), "5A1 9A1A1 9A9A2A");
}

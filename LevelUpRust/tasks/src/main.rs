use tasks::run_length_encoding::RunLengthEncoding;

fn main() {
    let decoded_string = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    let encoded_string = RunLengthEncoding::encode(decoded_string);

    println!(
        "{}",
        decoded_string == RunLengthEncoding::decode(&encoded_string)
    );
    println!("{}", encoded_string);
}

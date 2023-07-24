use chrono::NaiveDate;
use tasks::{
    handle_inconsistent_dates::flexible_date_parse, run_length_encoding::RunLengthEncoding,
};

fn main() {
    let decoded_string = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    let encoded_string = RunLengthEncoding::encode(decoded_string);

    println!(
        "{}",
        decoded_string == RunLengthEncoding::decode(&encoded_string)
    );
    println!("{}", encoded_string);

    println!("{:?}", NaiveDate::parse_from_str("1999/Mar/02", "%Y/%b/%d"));
    println!("{:?}", flexible_date_parse("1999/Mar/02"))
}

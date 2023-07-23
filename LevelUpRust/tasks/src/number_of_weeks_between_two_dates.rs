use chrono::NaiveDate;

pub fn number_of_weeks_between_two_dates(a: &str, b: &str) -> i64 {
    let fmt = "%F";

    (NaiveDate::parse_from_str(b, fmt).unwrap() - NaiveDate::parse_from_str(a, fmt).unwrap())
        .num_weeks()
}

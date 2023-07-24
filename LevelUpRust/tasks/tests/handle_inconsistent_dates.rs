use chrono::NaiveDate;
use tasks::handle_inconsistent_dates::*;

#[test]
fn ymd_hyphen() {
    assert_eq!(
        flexible_date_parse("2010-12-11"),
        Some(NaiveDate::from_ymd_opt(2010, 12, 11).unwrap())
    )
}

#[test]
fn ymd_slash() {
    assert_eq!(
        flexible_date_parse("1999/Mar/02"),
        Some(NaiveDate::from_ymd_opt(1999, 3, 2).unwrap())
    )
}

#[test]
fn dmy_dot() {
    assert_eq!(
        flexible_date_parse("01.Mar.2021"),
        Some(NaiveDate::from_ymd_opt(2021, 3, 1).unwrap())
    )
}

#[test]
fn mdy_dot() {
    assert_eq!(
        flexible_date_parse("Apr.05.2021"),
        Some(NaiveDate::from_ymd_opt(2021, 4, 5).unwrap())
    )
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}

use chrono::NaiveDate;

pub fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let fmts = vec![
        "%D",
        "%x",
        "%F",
        "%v",
        "%Y-%m-%d",
        "%Y/%b/%d",
        "%d.%b.%Y",
        "%b.%d.%Y",
        "%Y/%B%/%d",
        "%d.%B.%Y",
        "%b.%B.%Y",
    ];
    for fmt in fmts {
        if let Ok(d) = NaiveDate::parse_from_str(text, fmt) {
            return Some(d);
        } else {
            continue;
        }
    }
    None
}

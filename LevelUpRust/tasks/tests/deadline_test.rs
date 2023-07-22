use chrono::{Duration, Local};
use tasks::deadline::*;

#[test]
fn in_past() {
    let event = ImportantEvent::new(
        "Friend's birthday",
        Local::now().date_naive() - Duration::hours(25),
    );

    assert!(event.is_passed())
}

#[test]
fn in_future() {
    let event = ImportantEvent::new(
        "Friend's birthday",
        Local::now().date_naive() + Duration::hours(25),
    );

    assert!(!event.is_passed())
}

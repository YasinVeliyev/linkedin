use chrono::prelude::*;

#[derive(Debug)]
pub struct ImportantEvent {
    pub event_name: String,
    pub event_date: NaiveDate,
}

impl ImportantEvent {
    pub fn new(name: &str, event_date: NaiveDate) -> Self {
        Self {
            event_name: name.to_owned(),
            event_date,
        }
    }
}

pub trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.event_date < Local::now().date_naive()
    }
}

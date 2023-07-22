use chrono::Local;
use tasks::deadline::*;
fn main() {
    let _ = ImportantEvent::new("Ad", Local::now().date_naive());
}

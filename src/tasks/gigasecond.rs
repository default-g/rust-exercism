use time::PrimitiveDateTime as DateTime;
use time::Duration;

#[allow(dead_code)]
// Gigasecond
fn after(start: DateTime) -> DateTime {
    start.checked_add(Duration::seconds(1_000_000_000)).unwrap()
}



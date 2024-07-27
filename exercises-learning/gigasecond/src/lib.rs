use time::{Duration, PrimitiveDateTime as DateTime};


// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    const GIGA_SECONDS: i64 = 1_000_000_000; 
    // todo!("What time is a gigasecond later than {start}");
    start.checked_add(Duration::new(GIGA_SECONDS, 0)).unwrap()
}

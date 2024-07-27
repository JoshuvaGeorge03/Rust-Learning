use time_clock::Clock;

#[test]
fn on_the_hour() {
    assert_eq!(Clock::new(8, 10).toString(), "08:00");
}

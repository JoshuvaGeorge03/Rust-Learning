use std::env::args;

// extern crate time_clock;

// use time_clock::Clock;

use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: i64,
    minutes: i64,
}

fn sanitize_time(hours: i64, minutes: i64) -> (i64, i64) {
    let hours_converted_from_minutes = minutes.div_euclid(60);
    let remaining_number_of_minutes = minutes % 60;

    let mut new_hours = (hours + hours_converted_from_minutes) % 24;

    let mut new_minutes = remaining_number_of_minutes;

    if new_minutes.is_negative() {
        new_minutes += 60
    }

    if new_hours.is_negative() {
        new_hours += 24;
    }

    (new_hours, new_minutes)
}

// fn sanitize_time(hours: i64, minutes: i64) -> (i64, i64) {
//     let mut new_minutes: i64 = 0;
//     let mut new_hours: i64 = 0;

//     if minutes >= 0 && minutes < 60 {
//         new_minutes = minutes;
//     }

//     let new_minutes_in_hours = minutes / 60;

//     new_minutes = minutes % 60;

//     new_hours = hours + new_minutes_in_hours;

//     if new_hours >= 24 {
//         new_hours = new_hours % 24;
//     }

//     if new_hours < 0 {
//         let hours_within_twenty_four = new_hours % 24;
//         new_hours = hours_within_twenty_four + 24;
//     }

//     (new_hours, new_minutes)
// }

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        let (hours, minutes) = sanitize_time(hours, minutes);
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        let new_minutes = self.minutes + minutes;
        let (hours, minutes) = sanitize_time(self.hours, new_minutes);
        Self { hours, minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

fn main() {
    println!("args passed {:?}", args());

    let input_args_value = args().nth(1);

    println!("input args value {input_args_value:?}");

    let input_number_before_parsed = match input_args_value {
        Some(string) => string,
        None => "0".to_string(),
    };

    println!("input_number_before_parsed {input_number_before_parsed}");

    let input_number = input_number_before_parsed
        .parse::<u64>()
        .expect("Please provide correct input number");

    println!("value of division {}", input_number / 60);

    println!("value of modulus {}", input_number % 60);

    // println!("time clock value {:?}", Clock::new(-1, 15));
    assert_eq!(Clock::new(1, 61).to_string(), "02:01");
    assert_eq!(Clock::new(50, 180).to_string(), "05:00");
    assert_eq!(Clock::new(-22, 200).to_string(), "05:20");
    assert_eq!(Clock::new(-34, -200).to_string(), "10:40");
    assert_eq!(Clock::new(-9, 200).to_string(), "18:20");

    assert_eq!(Clock::new(-9, 200).add_minutes(600).to_string(), "04:20");

    assert_eq!(Clock::new(-54, -11_513).to_string(), "18:07");

    assert_eq!(Clock::new(15, 37), Clock::new(15, 37));

    assert_ne!(Clock::new(15, 36), Clock::new(15, 37));
    assert_eq!(Clock::new(-2, 40), Clock::new(22, 40));
    assert_eq!(Clock::new(-31, 3), Clock::new(17, 3));

    assert_eq!(Clock::new(24, 0), Clock::new(0, 0));

    // assert_eq!(Clock::new(-54, -11_513), Clock::new(18, 7));
}

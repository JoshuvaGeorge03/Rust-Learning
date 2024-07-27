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

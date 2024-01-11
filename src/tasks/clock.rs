use std::fmt;

#[derive(fmt::Debug)]
pub struct Clock {
   seconds_since_midnight: i32,
}

impl Clock {
    const SECONDS_PER_DAY: i32 = 60 * 60 * 24;

    pub fn hours_and_minutes_to_seconds(hours: i32, minutes: i32) -> i32 {
        let mut seconds = hours * 60 * 60 + minutes * 60;
        while seconds < 0 {
            seconds += Self::SECONDS_PER_DAY;
        }

        seconds
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {seconds_since_midnight: Self::hours_and_minutes_to_seconds(hours, minutes) % Self::SECONDS_PER_DAY}
    }

    #[allow(dead_code)]
    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes_to_seconds = minutes * 60;
        let total_seconds = self.seconds_since_midnight + minutes_to_seconds;

        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;

        Self::new(hours, minutes)
    }

    pub fn seconds_to_hours_and_minutes(seconds: i32) -> (i32, i32) {
        (seconds / 3600, (seconds % 3600) / 60)
    }
}


impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = Self::seconds_to_hours_and_minutes(self.seconds_since_midnight);
        write!(f, "{:02}:{:02}", time.0, time.1)
    }
}

impl std::cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.seconds_since_midnight == other.seconds_since_midnight
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

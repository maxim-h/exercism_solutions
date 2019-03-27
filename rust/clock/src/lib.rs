use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {hours: (hours + minutes/60) % 24,minutes: minutes % 60}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {hours:(self.hours + (60 + self.minutes + minutes)/60 - 1) % 24, minutes: (60 + (self.minutes + minutes) % 60) % 60}
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

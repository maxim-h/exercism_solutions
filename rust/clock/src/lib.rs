use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

fn div_floor(x: i32, divisor: i32) -> i32 {
    (divisor + x % divisor) % divisor
}


fn rem_floor(x: i32, modulus: i32) -> i32 {
    (x as f32 / modulus as f32).floor() as i32
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {hours: div_floor(hours + rem_floor(minutes, 60), 24), minutes: div_floor(minutes, 60)}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

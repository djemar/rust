use std::fmt::{Display, Formatter};
#[derive(Debug)]

pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0fill$}:{:0fill$}", self.hours, self.minutes, fill=2)
    }
}

impl PartialEq<Self> for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours: (hours.rem_euclid(24) +  ((minutes as f32)/60f32).floor() as i32 + 24)
            .rem_euclid(24), minutes: i32::abs(minutes%60+60)%60 }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_minutes = self.minutes + minutes;
        Clock { hours: i32::abs(( self.hours + ((new_minutes as f32)/60f32).floor() as i32 + 24)
            .rem_euclid(24)), minutes: i32::abs(new_minutes%60+60)%60}
    }
}

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let actual_minutes: i32 = minutes % 60;
        let actual_hours: i32 = hours + minutes / 60;
        Clock {
            hours: actual_hours,
            minutes: actual_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let actual_minutes: i32 = (self.minutes + minutes) % 60;
        let actual_hours: i32 = self.hours + (self.minutes + minutes) / 60;
        Clock {
            hours: actual_hours,
            minutes: actual_minutes,
        }
    }
}

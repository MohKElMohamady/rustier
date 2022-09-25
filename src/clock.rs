use std::fmt::{Display};
use crate::clock_errors::{ClockError};
use crate::daylight_saving::DaylightSaving;

pub struct Clock {
    hours : u64,
    minutes: u64,
    daylight_saving: DaylightSaving
}
impl Clock {
    pub fn midnight() -> Clock {
        Self { hours: 0, minutes: 0, daylight_saving: DaylightSaving::Unknown }
    }
    pub fn new(hour : u64, minutes: u64, daylight_saving: DaylightSaving) -> Result<Clock, ClockError> {
        if hour > 23 {
            return Err(ClockError::HoursTooBig)
        }
        if minutes > 59 {
            return Err(ClockError::MinutesTooBig)
        }
        Ok(Self {
             hours: hour,
             minutes,
             daylight_saving
        })
    }
    pub fn set_daylight_saving(&mut self, daylight_saving : DaylightSaving) {
        self.daylight_saving = daylight_saving
    }
    pub fn get_daylight_saving(&self) -> DaylightSaving {
        self.daylight_saving.clone()
    }
    pub fn increment(&mut self) {
        if self.minutes == 59 {
            if self.hours == 23 {
                self.hours = 0;
                self.minutes = 0;
            }
            self.minutes = 0
        }else {
            self.minutes = self.minutes + 1
        }
    }
}
impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes && self.daylight_saving == other.daylight_saving
    }
}
impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours_displayed = match self.hours {
            0 => format!("00"),
            1..=9 => format!("0{}", self.hours),
            10..=23 => format!("{}", self.hours),
            _ => panic!("Hours cannot exceed 23")
        };
        let minutes_displayed = match self.minutes {
            0 => format!("00"),
            1..=9 => format!("0{}", self.minutes),
            10..=59 => format!("{}", self.minutes),
            _ => panic!("Hours cannot exceed 59")
        };
        write!(f, "{}:{}", hours_displayed, minutes_displayed)
    }
}
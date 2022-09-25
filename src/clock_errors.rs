
use std::fmt::Display;

use thiserror::Error;

#[derive(Error,Debug)]
pub enum ClockError {
    HoursTooBig,
    MinutesTooBig   
}

impl Display for ClockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClockError::HoursTooBig => f.write_str("The number of hours cannot exceed 23"),
            ClockError::MinutesTooBig => f.write_str("The number of minutes cannot exceed 59"),
        }
    }
}
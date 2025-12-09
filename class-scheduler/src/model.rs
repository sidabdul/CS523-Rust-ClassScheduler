use crate::day::Day;
use crate::time::Time;

#[derive(Debug, Clone)]
pub struct Meeting {
    pub day: Day,
    pub start: Time,
    pub end: Time,
    pub location: String,
}

#[derive(Debug, Clone)]
pub struct Course {
    pub code: String,
    pub title: String,
    pub meetings: Vec<Meeting>,
}


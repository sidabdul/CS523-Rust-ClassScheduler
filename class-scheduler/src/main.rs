// Sid A.
// CS523 Rust - This is the main file for the Rust School Scheduler app.
use std::collections::BTreeMap;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Day {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl Day {
    fn short(&self) -> &'static str {
        match self {
            Day::Mon => "Mon",
            Day::Tue => "Tue",
            Day::Wed => "Wed",
            Day::Thu => "Thu",
            Day::Fri => "Fri",
            Day::Sat => "Sat",
            Day::Sun => "Sun",
        }
    }
    fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "mon" | "monday" => Some(Day::Mon),
            "tue" | "tuesday" => Some(Day::Tue),
            "wed" | "wednesday" => Some(Day::Wed),
            "thu" | "thursday" => Some(Day::Thu),
            "fri" | "friday" => Some(Day::Fri),
            "sat" | "saturday" => Some(Day::Sat),
            "sun" | "sunday" => Some(Day::Sun),
            _ => None,
        }
    }
    fn all() -> [Day; 7] {
        [
            Day::Mon,
            Day::Tue,
            Day::Wed,
            Day::Thu,
            Day::Fri,
            Day::Sat,
            Day::Sun,
        ]
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
struct Time(u16);
impl Time {
    fn parse(s: &str) -> Result<Self, String> {
        let mut it = s.split(':');
        let h = it
            .next()
            .ok_or("time must be HH:MM")?
            .parse::<u16>()
            .map_err(|_| "bad hour")?;
        let m = it
            .next()
            .ok_or("time must be HH:MM")?
            .parse::<u16>()
            .map_err(|_| "bad minute")?;
        if h > 23 || m > 59 {
            return Err("hour/minute out of range".into());
        }
        Ok(Time(h * 60 + m))
    }
    fn fmt(self) -> String {
        let h = self.0 / 60;
        let m = self.0 % 60;
        format!("{:02}:{:02}", h, m)
    }
}
#[derive(Debug, Clone)]
struct Meeting {
    day: Day,
    start: Time,
    end: Time,
    location: String,
}

impl Meeting {
    fn overlaps(&self, other: &Meeting) -> bool {
        if self.day != other.day {
            return false;
        }
        !(self.end.0 <= other.start.0 || other.end.0 <= self.start.0)
    }
}
#[derive(Debug, Clone)]
struct Class {
    code: String,
    title: String,
    meetings: Vec<Meeting>,
}
#[derive(Default)]
struct Schedule {
    classes: Vec<Class>,
}

fn help() {
    println!(
        r#"Commands:
 add-class <CODE> <TITLE...>
 add-meeting <CODE> <DAY> <START> <END> <LOCATION...>
 remove-class <CODE>
 list
 week
 conflicts
 help
 quit"#
    );
}

impl Schedule {
    fn add_class(&mut self, code: &str, title: &str) -> Result<(), String> {
        if self
            .classes
            .iter()
            .any(|c| c.code.eq_ignore_ascii_case(code))
        {
            return Err(format!("Class '{}' already exists.", code));
        }
        self.classes.push(Class {
            code: code.to_string(),
            title: title.to_string(),
            meetings: vec![],
        });
        Ok(())
    }


         fn find_class_mut(&mut self, code: &str) -> Option<&mut Class>{
        self.classes
            .iter_mut()
            .find(|c| c.code.eq_ignore_ascii_case(code))
    }




}


fn main() {
    println!("Hello, world!");
}

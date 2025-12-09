/*
 * Sid A Rust Class scheduler

	This Project is a class scheduler tool. This tool would allow a student to register for a term of classes same way we currently use Ban web here at Portland State to register for classes. I want to work on this assignment because I find it interesting how this system is currently in place on our schools website. The project touches on multiple areas: task scheduling, time-management , data analytics, and CLI uasge.
 *
 *
 */

use std::collections::BTreeMap;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Time(u16);

impl Time {
    fn parse(s: &str) -> Result<Self, String> {
        let mut sections = s.split(':');
        let h = sections
            .next()
            .ok_or_else(|| "time must be HH:MM".to_string())?
            .parse::<u16>()
            .map_err(|_| "wrong hour".to_string())?;
        let m = sections
            .next()
            .ok_or_else(|| "time must be HH:MM".to_string())?
            .parse::<u16>()
            .map_err(|_| "wrong minute".to_string())?;
        if h > 23 || m > 59 {
            return Err("hour and or is minute out of range".into());
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

impl Schedule {
    fn add_class(&mut self, code: &str, title: &str) -> Result<(), String> {
        if self
            .classes
            .iter()
            .any(|c| c.code.eq_ignore_ascii_case(code))
        {
            return Err(format!("Class '{code}' already exists."));
        }
        self.classes.push(Class {
            code: code.to_string(),
            title: title.to_string(),
            meetings: vec![],
        });
        Ok(())
    }

    fn delete_class(&mut self, code: &str) -> bool {
        let before = self.classes.len();
        self.classes.retain(|c| !c.code.eq_ignore_ascii_case(code));
        before != self.classes.len()
    }

    fn find_class_mut(&mut self, code: &str) -> Option<&mut Class> {
        self.classes
            .iter_mut()
            .find(|c| c.code.eq_ignore_ascii_case(code))
    }

    fn all_meetings(&self) -> Vec<(&Class, &Meeting)> {
        self.classes
            .iter()
            .flat_map(|c| c.meetings.iter().map(move |m| (c, m)))
            .collect()
    }

    fn list(&self) {
        if self.classes.is_empty() {
            println!("(no classes)");
            return;
        }

        for c in &self.classes {
            println!("{} — {}", c.code, c.title);

            if c.meetings.is_empty() {
                println!("  (no meetings!)");
            } else {
                for (i, m) in c.meetings.iter().enumerate() {
                    println!(
                        "  [{:>2}] {} {}-{} @ {}",
                        i + 1,
                        m.day.short(),
                        m.start.fmt(),
                        m.end.fmt(),
                        m.location
                    );
                }
            }
        }
    }

    fn week(&self) {
        let mut map: BTreeMap<Day, Vec<(&Class, &Meeting)>> = BTreeMap::new();
        for d in Day::all() {
            map.entry(d).or_default();
        }

        for c in &self.classes {
            for m in &c.meetings {
                map.get_mut(&m.day).expect("day exists").push((c, m));
            }
        }

        for d in Day::all() {
            println!("{}:", d.short());
            let v = map.get_mut(&d).expect("day exists");
            v.sort_by_key(|(_, m)| (m.start, m.end));
            if v.is_empty() {
                println!(" (none)");
            } else {
                for (c, m) in v.iter() {
                    println!(
                        " {:>5}-{:>5} {:<8} {:<} @ {}",
                        m.start.fmt(),
                        m.end.fmt(),
                        c.code,
                        c.title,
                        m.location
                    );
                }
            }
            println!();
        }
    }

        fn help(&self) {
    println!(
        r#"Commands:
 Add-a-class <CODE> <TITLE...> For example .. CS423 Rust
 Add-a-meeting  <CODE> <DAY> <START> <END> <LOCATION...> For example .. CS101 Mon 08:30 10:30 EB101
 Remove-a-class <CODE> For example .. CS101
 List
 Week
 Conflicts
 Menu
 quit"#
    );
}


    fn conflicts(&self) {
        let pairs = self.all_meetings();
        let mut found = false;
        for i in 0..pairs.len() {
            for j in (i + 1)..pairs.len() {
                let (c1, m1) = pairs[i];
                let (c2, m2) = pairs[j];
                if m1.overlaps(m2) {
                    found = true;
                    println!(
                        "Conflict on {}: {} {}-{} @ {} <-> {} {}-{} @ {}",
                        m1.day.short(),
                        c1.code,
                        m1.start.fmt(),
                        m1.end.fmt(),
                        m1.location,
                        c2.code,
                        m2.start.fmt(),
                        m2.end.fmt(),
                        m2.location
                    );
                }
            }
        }
        if !found {
            println!("No conflicts found.");
        }
    }



}

fn menu() {
    println!(
        r#"Commands:
 Add-a-class <CODE> <TITLE...>
 Add-a-meeting  <CODE> <DAY> <START> <END> <LOCATION...>
 Remove-a-class <CODE>
 List
 Week
 Conflicts
 Menu
 Help(For syntax help/format)
 quit"#
    );
}


fn main() {
    let mut sched = Schedule::default();
    println!("Welcome to the Class School Scheduler! Please Type 'Menu' to get started...");

    loop {
        print!("-> ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("Failed to flush stdout: {e}");
        }

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut it = line.split_whitespace();
        let cmd = it.next().unwrap_or("");

        match cmd {
            "Menu" => menu(),
            "quit" | "exit" => {
                println!("See you later! Bye!");
                break;
            }
            "Add-a-class" => {
                let code = match it.next() {
                    Some(s) => s,
                    None => {
                        eprintln!("Usage: Add-a-class <CODE> <TITLE...>");
                        continue;
                    }
                };
                let title = it.collect::<Vec<_>>().join(" ").replace('_', " ");
                if title.is_empty() {
                    eprintln!("Title cannot be empty.");
                    continue;
                }
                match sched.add_class(code, &title) {
                    Ok(_) => println!("Added class: {} — {}", code, title),
                    Err(e) => eprintln!("{e}"),
                }
            }
            "Add-a-meeting" => {
                let code = match it.next() {
                    Some(s) => s,
                    None => {
                        eprintln!("Usage: Add-a-meeting <CODE> <DAY> <START> <END> <LOCATION...>");
                        continue;
                    }
                };
                let day = match it.next().and_then(Day::parse) {
                    Some(d) => d,
                    None => {
                        eprintln!("Bad or missing day (use Mon/Tue/... or Monday/etc).");
                        continue;
                    }
                };
                let start = match it.next().map(Time::parse) {
                    Some(Ok(t)) => t,
                    _ => {
                        eprintln!("Bad or missing start time (use HH:MM).");
                        continue;
                    }
                };
                let end = match it.next().map(Time::parse) {
                    Some(Ok(t)) => t,
                    _ => {
                        eprintln!("Bad or missing end time (use HH:MM).");
                        continue;
                    }
                };
                if end.0 <= start.0 {
                    eprintln!("End time must be after start time.");
                    continue;
                }
                let location = it.collect::<Vec<_>>().join(" ").replace('_', " ");
                if location.is_empty() {
                    eprintln!("Location must not be empty.");
                    continue;
                }

                if let Some(class) = sched.find_class_mut(code) {
                    let m = Meeting {
                        day,
                        start,
                        end,
                        location,
                    };
                    class.meetings.push(m);
                    println!("Added meeting to {}.", class.code);
                } else {
                    eprintln!("No such class: {code}");
                }
            }
            "Remove-a-class" => {
                let code = match it.next() {
                    Some(s) => s,
                    None => {
                        eprintln!("Usage: Remove-a-class <CODE>");
                        continue;
                    }
                };
                if sched.delete_class(code) {
                    println!("Removed '{code}'");
                } else {
                    eprintln!("No such class: {code}");
                }
            }
            "List" => sched.list(),
            "Week" => sched.week(),
            "Conflicts" => sched.conflicts(),
            "Help" => sched.help(),
            _ => eprintln!("Unknown command (type 'Menu')."),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_class() {
        let mut s = Schedule::default();

        // Add CS101 to test...
        let ok = s.add_class("CS101", "Intro to CS");
        assert!(ok.is_ok());
        assert_eq!(s.classes.len(), 1);
        assert_eq!(s.classes[0].code, "CS101");
    }

    #[test]
    fn test_delete_class() {
        let mut s = Schedule::default();

        s.add_class("CS423", "Rust").unwrap();
        assert_eq!(s.classes.len(), 1);

        let removed_class = s.delete_class("CS423");
        assert!(removed_class);
        assert_eq!(s.classes.len(), 0);
    }
}

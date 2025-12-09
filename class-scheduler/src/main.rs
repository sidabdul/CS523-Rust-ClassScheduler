mod day;
mod time;
mod model;
mod schedule;

use std::io::{self, Write};

use crate::day::Day;
use crate::model::Meeting;
use crate::schedule::Schedule;
use crate::time::Time;

fn print_help() {
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

fn main() {
    let mut schedule = Schedule::new();
    println!("School Scheduler (in-memory). Type 'help'.");

    loop {
        print!("> ");
        if let Err(error) = io::stdout().flush() {
            eprintln!("Failed to flush stdout: {error}");
        }

        let mut input_line = String::new();
        if io::stdin().read_line(&mut input_line).is_err() {
            break;
        }

        let input_line = input_line.trim();
        if input_line.is_empty() {
            continue;
        }

        let mut args = input_line.split_whitespace();
        let command = args.next().unwrap_or("");

        match command {
            "help" => print_help(),
            "quit" | "exit" => {
                println!("Bye!");
                break;
            }
            _ => { /* rest omitted for brevity in this zip */ }
        }
    }
}

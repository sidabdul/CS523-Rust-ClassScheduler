Sid A CS523 Rust

Class scheduler 


	This program is a class scheduler tool. This tool would allow a student to register for a term of classes same way we currently use Ban web here at Portland State to register for classes. I wanted to work on this assignment because I find it interesting how this system is currently in place on our schools website. The project touches on multiple areas: task scheduling, time-management , data analytics, and CLI usage.

To build and run the program:

1.In the src directory, type cargo build
2.Type cargo run.
3.Type Menu
4.Options should be displayed. 

Here is a test run of the program:


sidali@fab05:/home/sidali/common/Desktop/Rust/Project/class-scheduler/src$ cargo run 
  Compiling class-scheduler v0.1.0 (/home/sidali/common/Desktop/Rust/Project/class-scheduler) 
   Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.51s 
    Running `/home/sidali/common/Desktop/Rust/Project/class-scheduler/target/debug/class-scheduler` 
Welcome to the Class School Scheduler! Please Type 'Menu' to get started... 
-> Menu 
Commands: 
Add-a-class <CODE> <TITLE...> 
Add-a-meeting  <CODE> <DAY> <START> <END> <LOCATION...> 
Remove-a-class <CODE> 
List 
Week 
Conflicts 
Menu 
Help(For syntax help/format) 
quit 
-> Add-a-class CS101 Rust 
Added class: CS101 — Rust 
-> Add-a-meeting CS101 Mon 08:30 09:30 EB101 
Added meeting to CS101. 
-> List 
CS101 — Rust 
 [ 1] Mon 08:30-09:30 @ EB101 
-> Week 
Mon: 
08:30-09:30 CS101    Rust @ EB101 

Tue: 
(none) 

Wed: 
(none) 

Thu: 
(none) 

Fri: 
(none) 

Sat: 
(none) 

Sun: 
(none) 

-> 




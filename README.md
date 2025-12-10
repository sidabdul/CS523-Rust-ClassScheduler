# Sid A CS523 Rust - Class scheduler

This program is a class scheduler tool. This tool would allow a student to register for a term of classes same way we currently use Ban web here at Portland State to register for classes. I wanted to work on this assignment because I find it interesting how this system is currently in place on our schools website. The project touches on multiple areas: task scheduling, time-management , data analytics, and CLI usage.


# To build and run the program:

1. In the src directory, type cargo build 
2. Type cargo run. 
3. Type Menu 
4. Options should be displayed.

## Here is a test run of the program:

sidali@fab05:/home/sidali/common/Desktop/Rust/Project/class-scheduler/src$ cargo run   

Compiling class-scheduler v0.1.0 (/home/sidali/common/Desktop/Rust/Project/class-scheduler)    Finished dev profile [unoptimized + debuginfo] target(s) in 1.51s     

Running /home/sidali/common/Desktop/Rust/Project/class-scheduler/target/debug/class-scheduler 

Welcome to the Class School Scheduler! Please Type 'Menu' to get started... 
-> Menu 

Please select one of the options below:
 1. Add-a-class <2 Letter class CODE> + <TITLE...> For example .. CS423 Rust
 2. Add-a-meeting  <2 Letter class CODE> <DAY> <START> <END> <LOCATION...> For example .. CS101 Mon 08:30 10:30 EB101
 3. Remove-a-class <2 Letter class CODE> + <TITLE...>
 4. List
 5. Week
 6. Conflicts
 7. Menu
 8. Help(For syntax help/format)
 9. quit
-> 


-> Add-a-class CS101 Rust
Added class: CS101 — Rust

-> Add-a-meeting CS101 Mon 08:30 09:30 EB101
Added meeting to CS101.

-> List
CS101 — Rust
 [ 1] Mon 08:30-09:30 @ EB101

-> Week
Mon:
08:30-09:30 CS101    Rust @ EB101

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

->quit



## Testing

I added some testing to this program as well. The test ensures a class can be added and deleted:

idali@fab05:/home/sidali/common/Desktop/Rust/Project/class-scheduler$ cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/main.rs (target/debug/deps/class_scheduler-da2d92ff691641de)

running 2 tests
test tests::test_delete_class ... ok
test tests::test_add_class ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s




use std::fs;

pub fn run_day_1() {
    println!("Day 1");

    let contents = fs::read_to_string("assets/day1.txt")
        .expect("Should have been able to read the file");

    for line in contents.lines() {
        println!("{}", line);
    }
}
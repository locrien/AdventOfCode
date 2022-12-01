use std::io;

fn main() {
    println!("which day to run?");

    let mut day = String::new();
    io::stdin().read_line(&mut day).expect("Failed to read");

    let day: u32 = if let Ok(num) = day.trim().parse() {
        num
    } else {
        println!("Invalid input");
        1
    };

    match day {
        1 => day_1(),
        _ => println!("Invalid day"),
    }
}

fn day_1() {
    println!("Day 1");
}
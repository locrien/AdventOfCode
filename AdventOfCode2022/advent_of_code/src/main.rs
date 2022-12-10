use std::io;

mod days;

fn main() 
{
    println!("which day to run?");

    let mut day = String::new();
    io::stdin().read_line(&mut day).expect("Failed to read day");

    let day:u32 = day.trim().parse().expect("failed to parse day");

    println!("which part to run?");

    let mut part = String::new();
    io::stdin().read_line(&mut part).expect("Failed to read part");

    let part:u32 = part.trim().parse().expect("failed to parse part");

    days::run(day, Some(part));
}
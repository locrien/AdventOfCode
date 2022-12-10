use std::io;

mod days;

fn main() {
    println!("which day to run?");

    let mut day = String::new();
    io::stdin().read_line(&mut day).expect("Failed to read day");

    let day:u32 = day.trim().parse().expect("failed to parse day");

    println!("which part to run?");

    let mut part = String::new();
    io::stdin().read_line(&mut part).expect("Failed to read part");

    let part:u32 = part.trim().parse().expect("failed to parse part");

    match (day, part)  
    {
        (1,1) => _ = days::day1::run_part_1("assets/day1.txt"),
        (1,2) => _ = days::day1::run_part_2("assets/day1.txt"),
        (2,1) => _ = days::day2::run_part_1("assets/day2.txt"),
        (2,2) => _ = days::day2::run_part_2("assets/day2.txt"),
        (3,1) => _ = days::day3::run_part_1("assets/day3.txt"),
        (3,2) => _ = days::day3::run_part_2("assets/day3.txt"),
        (4,1) => _ = days::day4::run_part_1("assets/day4.txt"),
        (4,2) => _ = days::day4::run_part_2("assets/day4.txt"),
        (5,1) => _ = days::day5::run_part_1("assets/day5.txt"),
        (5,2) => _ = days::day5::run_part_2("assets/day5.txt"),
        (6,1) => _ = days::day6::run_part_1("assets/day6.txt"),
        (6,2) => _ = days::day6::run_part_2("assets/day6.txt"),
        _ => println!("Invalid day/part"),
    }    
}
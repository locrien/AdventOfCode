use std::{io, fs};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn run(day:u32, part: Option<u32>)
{
    let part = match part
    {
        Some(val) => val,
        None => query_part(),
    };

    let path = format!("assets/{}", day);
    let input = fs::read_to_string(path)
    .expect("Should have been able to read the file");

    match day  
    {
        1 => day1::run(input, part),
        2 => day2::run(input, part),
        3 => day3::run(input, part),
        4 => day4::run(input, part),
        5 => day5::run(input, part),
        6 => day6::run(input, part),
        7 => day7::run(input, part),
        8 => day8::run(input, part),
        9 => day9::run(input, part),
        10 => day10::run(input, part),
        11 => day11::run(input, part),
        12 => day12::run(input, part),
        13 => day13::run(input, part),
        14 => day14::run(input, part),
        15 => day15::run(input, part),
        16 => day16::run(input, part),
        17 => day17::run(input, part),
        18 => day18::run(input, part),
        19 => day19::run(input, part),
        20 => day20::run(input, part),
        21 => day21::run(input, part),
        22 => day22::run(input, part),
        23 => day23::run(input, part),
        24 => day24::run(input, part),
        25 => day25::run(input, part),
        _ => println!("Invalid day"),
    }
}

fn query_part() -> u32
{
    let mut part = String::new();
    io::stdin().read_line(&mut part).expect("Failed to read part");

    return part.trim().parse().expect("failed to parse part");
}
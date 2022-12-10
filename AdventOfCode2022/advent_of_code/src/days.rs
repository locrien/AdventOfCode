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

    match (day, part)  
    {
        (1,1) => _ = day1::run_part_1(input),
        (1,2) => _ = day1::run_part_2(input),
        (2,1) => _ = day2::run_part_1(input),
        (2,2) => _ = day2::run_part_2(input),
        (3,1) => _ = day3::run_part_1(input),
        (3,2) => _ = day3::run_part_2(input),
        (4,1) => _ = day4::run_part_1(input),
        (4,2) => _ = day4::run_part_2(input),
        (5,1) => _ = day5::run_part_1(input),
        (5,2) => _ = day5::run_part_2(input),
        (6,1) => _ = day6::run_part_1(input),
        (6,2) => _ = day6::run_part_2(input),
        (7,1) => _ = day7::run_part_1(input),
        (7,2) => _ = day7::run_part_2(input),
        (8,1) => _ = day8::run_part_1(input),
        (8,2) => _ = day8::run_part_2(input),
        (9,1) => _ = day9::run_part_1(input),
        (9,2) => _ = day9::run_part_2(input),
        (10,1) => _ = day10::run_part_1(input),
        (10,2) => _ = day10::run_part_2(input),
        (11,1) => _ = day11::run_part_1(input),
        (11,2) => _ = day11::run_part_2(input),
        (12,1) => _ = day12::run_part_1(input),
        (12,2) => _ = day12::run_part_2(input),
        (13,1) => _ = day13::run_part_1(input),
        (13,2) => _ = day13::run_part_2(input),
        (14,1) => _ = day14::run_part_1(input),
        (14,2) => _ = day14::run_part_2(input),
        (15,1) => _ = day15::run_part_1(input),
        (15,2) => _ = day15::run_part_2(input),
        (16,1) => _ = day16::run_part_1(input),
        (16,2) => _ = day16::run_part_2(input),
        (17,1) => _ = day17::run_part_1(input),
        (17,2) => _ = day17::run_part_2(input),
        (18,1) => _ = day18::run_part_1(input),
        (18,2) => _ = day18::run_part_2(input),
        (19,1) => _ = day19::run_part_1(input),
        (19,2) => _ = day19::run_part_2(input),
        (20,1) => _ = day20::run_part_1(input),
        (20,2) => _ = day20::run_part_2(input),
        (21,1) => _ = day21::run_part_1(input),
        (21,2) => _ = day21::run_part_2(input),
        (22,1) => _ = day22::run_part_1(input),
        (22,2) => _ = day22::run_part_2(input),
        (23,1) => _ = day23::run_part_1(input),
        (23,2) => _ = day23::run_part_2(input),
        (24,1) => _ = day24::run_part_1(input),
        (24,2) => _ = day24::run_part_2(input),
        (25,1) => _ = day25::run_part_1(input),
        (25,2) => _ = day25::run_part_2(input),
        _ => println!("Invalid day"),
    }
}

fn query_part() -> u32
{
    let mut part = String::new();
    io::stdin().read_line(&mut part).expect("Failed to read part");

    return part.trim().parse().expect("failed to parse part");
}
use std::{fs, vec};

pub fn run_day_1() {
    println!("Day 1");

    let mut v : Vec<u32> = Vec::new();
    v.push(0);

    let contents = fs::read_to_string("assets/day1.txt")
        .expect("Should have been able to read the file");

    for line in contents.lines() {

        let last_idx = v.len();

        match line.trim().parse::<u32>() {
            Ok(num) => {v[last_idx-1] += num},
            Err(_why) => v.push(0),
        }

    }

    v.sort();

    println!("1st {}", v[&v.len()-1]);
    println!("2nd {}", v[&v.len()-2]);
    println!("3rd {}", v[&v.len()-3]);

    println!("sum {}", v[&v.len()-1] + v[&v.len()-2] + v[&v.len()-3]);
}
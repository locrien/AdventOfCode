use std::io;

mod day_1;

fn main() {
    println!("which day to run?");

    let mut day = String::new();
    io::stdin().read_line(&mut day).expect("Failed to read");

    match day.trim().parse()  {
        Ok(1) => day_1::run_day_1(),
        _ => println!("Invalid day"),
    }
}
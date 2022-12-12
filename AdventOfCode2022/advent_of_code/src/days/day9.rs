use std::{collections::HashSet, borrow::BorrowMut};

#[cfg(test)]
mod tests
{
    use super::*;
    use std::fs;
    use const_format::formatcp;

    const DAY: u32= 9;
    const FILE_NAME: &str = formatcp!("assets/day{DAY}.txt");
    const TEST_FILE_NAME: &str = formatcp!("assets/tests/day{DAY}.txt");
    const TEST_FILE_NAME_2: &str = formatcp!("assets/tests/day{DAY}_2.txt");
    #[test]
    fn part_1_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 13;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    // not 3665
    #[test]
    fn part_1()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 6271;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME_2)
        .expect("Should have been able to read the file");

        let expected = 36;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 206582;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }
}

struct Step
{
    direction:char,
    amount:u32,
}

// ----------------------------
pub fn run_part_1(input:String) -> u32
{
    let mut visited_spaces:HashSet<(i32,i32)> = HashSet::new();

    let mut knots:Vec<(i32,i32)> = vec![(0,0), (0,0)];

    process_input(input).iter().for_each(|s| { perform_step(s,&mut knots, &mut visited_spaces)});

    visited_spaces.insert((0,0));
    
    let result = visited_spaces.len() as u32;

    println!("{}", result);
    return result;
}

pub fn run_part_2(input:String) -> u32
{
    let mut visited_spaces:HashSet<(i32,i32)> = HashSet::new();

    let mut knots:Vec<(i32,i32)> = vec![(0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0)];

    process_input(input).iter().for_each(|s| { perform_step(s,&mut knots, &mut visited_spaces)});

    visited_spaces.insert((0,0));
    
    let result = visited_spaces.len() as u32;

    println!("{}", result);
    return result;
}

fn process_input(input:String) -> Vec<Step>
{
    let result = input.lines()
    .map(|l| { 
        let split_line:Vec<&str> = l.split(' ').collect();

        let step = Step
        {
            direction : split_line[0].chars().last().expect("no direction"),
            amount : split_line[1].parse().expect("cant parse"),
        };
        return step 
    });

    return result.collect();
}

fn perform_step(step:&Step, knots:&mut Vec<(i32,i32)>, v:&mut HashSet<(i32,i32)>)
{
    let dir:(i32,i32) = match step.direction 
    {
        'U' => (0,1),
        'D' => (0,-1),
        'L' => (-1,0),
        'R' => (1,0),
        _ => (0,0),
    };

    for _ in 0..step.amount
    {
        knots[0].0 = knots[0].0 + dir.0;
        knots[0].1 = knots[0].1 + dir.1;

        //println!("head pos {} {}",knots[0].0,knots[0].1);

        for k in 1..knots.len()
        {
            let length = knots.len();
            let knot1 = knots.get(k).unwrap();
            let knot2 = knots.get(k-1).unwrap();

            let delta =  get_knot_movement(knot1,knot2);

            let mut knot1 = knots.get_mut(k).unwrap();
            knot1.0 = knot1.0 + delta.0;
            knot1.1 = knot1.1 + delta.1;

            if k == length - 1
            {
                v.insert((knot1.0,knot1.1));
            }

            //println!("knot {} pos {} {}",k, knots[k].0,knots[k].1);
        }
    }
}

fn get_knot_movement(current_pos:&(i32,i32), to_follow_pos:&(i32,i32)) -> (i32,i32)
{
    let diff:(i32,i32) = (to_follow_pos.0 - current_pos.0,to_follow_pos.1 - current_pos.1);

    // move tail
    if diff.0.abs() > 1 || diff.1.abs() > 1
    {
        let mut delta = (0,0);
        if diff.0.abs() > 0
        {
            delta.0 = 1 * diff.0.signum();
        }

        if diff.1.abs() > 0
        {
            delta.1 =  1 * diff.1.signum();
        }

        return delta;
    }

    return (0,0);
}
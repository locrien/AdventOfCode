use std::{fs, collections::HashSet};

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_part_1_test()
    {
        let expected = 7;
        let result = run_part_1("assets/day6_test.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_1()
    {
        let expected = 1198;
        let result = run_part_1("assets/day6.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_2_test()
    {
        let expected = 19;
        let result = run_part_2("assets/day6_test.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_2()
    {
        let expected = 3120;
        let result = run_part_2("assets/day6.txt");
        assert_eq!(result,expected);
    }
}

// ----------------------------


pub fn run_part_1(file_name:&str) -> u32
{
    return find_position(read_data(file_name).as_str(), 4);
}

pub fn run_part_2(file_name:&str) -> u32
{
    return find_position(read_data(file_name).as_str(), 14);
}

fn find_position(input: &str, length:usize) -> u32
{
    let idx = input.chars().collect::<Vec<char>>().windows(length)
    .position(|w| w.iter().collect::<HashSet<_>>().len() == length)
    .expect("") as u32;

    return idx + length as u32;
}

fn read_data(file_name:&str) -> String
{
    return fs::read_to_string(file_name)
    .expect("Should have been able to read the file");
}
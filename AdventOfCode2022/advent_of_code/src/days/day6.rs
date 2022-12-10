use std::collections::HashSet;

#[cfg(test)]
mod tests
{
    use super::*;
    use const_format::formatcp;
    use std::fs;

    const DAY: u32= 6;
    const FILE_NAME: &str = formatcp!("assets/day{DAY}.txt");
    const TEST_FILE_NAME: &str = formatcp!("assets/tests/day{DAY}.txt");
    #[test]
    fn part_1_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 7;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_1()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 1198;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 19;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 3120;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }
}

// ----------------------------
pub fn run_part_1(input:String) -> u32
{
    return find_position(input.as_str(), 4);
}

pub fn run_part_2(input:String) -> u32
{
    return find_position(input.as_str(), 14);
}

fn find_position(input: &str, length:usize) -> u32
{
    let idx = input.chars().collect::<Vec<char>>().windows(length)
    .position(|w| w.iter().collect::<HashSet<_>>().len() == length)
    .expect("") as u32;

    return idx + length as u32;
}
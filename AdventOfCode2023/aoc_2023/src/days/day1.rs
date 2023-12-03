
#[cfg(test)]
mod tests
{
    use super::*;
    use const_format::formatcp;
    use std::fs;

    const DAY: u32= 1;
    const FILE_NAME: &str = formatcp!("assets/real/day{DAY}.txt");
    const TEST_FILE_NAME: &str = formatcp!("assets/tests/day{DAY}.txt");
    const TEST_FILE_NAME_P2: &str = formatcp!("assets/tests/day{DAY}_p2.txt");

    #[test]
    fn part_1_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 142;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_1()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 53080;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME_P2)
        .expect("Should have been able to read the file");

        let expected = 281;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 53268;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }
}

pub fn run_part_1(input:String) -> u32
{
    let mut calibrarions : Vec<u32> = Vec::new();
    read_calibrations(input, &mut calibrarions, true);
    
    let mut result = 0;
    for cal in calibrarions {
        result += cal;
    }

    println!("sum of calibration results : {}", result);

    return result;
}

pub fn run_part_2(input:String) -> u32
{
    let mut calibrarions : Vec<u32> = Vec::new();
    read_calibrations(input, &mut calibrarions, false);

    let mut result = 0;
    for cal in calibrarions {
        result += cal;
    }

    println!("sum of calibration results : {}", result);

    return result;
}

fn read_calibrations(input:String, calibrations: &mut Vec<u32>, part1: bool)
{
    for line in input.lines() 
    {
        let patterns = if part1 
        {vec![("1",1),("2",2),("3",3),("4",4),("5",5),("6",6),("7",7),("8",8),("9",9)]} else 
        {vec![("1",1),("2",2),("3",3),("4",4),("5",5),("6",6),("7",7),("8",8),("9",9),("one",1),("two",2), ("three",3), ("four",4), ("five",5), ("six",6), ("seven",7), ("eight",8), ("nine",9)]};

        let mut min = None::<(usize,u32)>;
        let mut max = None::<(usize,u32)>;

        for pattern in patterns {
            match line.find(pattern.0) {
                Some(r) => {
                    match min {
                        Some(m) => if r < m.0 {min = Some((r, pattern.1))},
                        None => {min = Some((r, pattern.1))},
                    }
                },
                None => {},
            }

            match line.rfind(pattern.0) {
                Some(r) => {
                    match max {
                        Some(m) => if r > m.0 {max = Some((r, pattern.1))},
                        None => {max = Some((r, pattern.1))},
                    }
                },
                None => {},
            }
        }
        
        match (min, max) {
            (Some(l), Some(r)) => {calibrations.push(l.1 * 10 + r.1)},
            _ => {},
        };
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use const_format::formatcp;
    use std::fs;

    const DAY: u32= 1;
    const FILE_NAME: &str = formatcp!("assets/day{DAY}.txt");
    const TEST_FILE_NAME: &str = formatcp!("assets/tests/day{DAY}.txt");

    #[test]
    fn part_1_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 24000;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_1()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 70116;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 45000;
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

pub fn run_part_1(input:String) -> u32
{
    let mut calories : Vec<u32> = Vec::new();
    read_calories(input, &mut calories);
    
    let result = *calories.last().expect("No elements found");
    println!("Elf with most calories transported : {}", result);

    return result;
}

pub fn run_part_2(input:String) -> u32
{
    let mut calories : Vec<u32> = Vec::new();
    read_calories(input, &mut calories);

    let result = calories[&calories.len()-3..].iter().sum::<u32>();
    println!("sum of top 3 : {}", result);

    return result;
}

fn read_calories(input:String, calories: &mut Vec<u32>)
{
    calories.push(0);

    for line in input.lines() 
    {
        let last_idx = calories.len();

        match line.trim().parse::<u32>() 
        {
            Ok(num) => {calories[last_idx-1] += num},
            Err(_why) => calories.push(0),
        }
    }

    calories.sort();
}
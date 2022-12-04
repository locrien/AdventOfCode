use std::fs;

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_part_1_test()
    {
        let expected = 157;
        let result = run_part_1("assets/day3_test.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_1()
    {
        run_part_1("assets/day3.txt");
    }

    #[test]
    fn test_part_2_test()
    {
        let expected = 0;
        let result = run_part_2("assets/day3_test.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_2()
    {
        run_part_2("assets/day3.txt");
    }
}

struct Rucksack
{
    pub left_compartment : String,
    pub right_compartment: String,
}

pub fn run_part_1(file_name:&str) -> i32
{
    let mut rucksacks : Vec<Rucksack> = Vec::new();
    read_data(file_name, &mut rucksacks);

    let result = 0;

    return 0;
    
}

pub fn run_part_2(file_name:&str) -> i32
{
    let mut rucksacks : Vec<Rucksack> = Vec::new();
    read_data(file_name, &mut rucksacks);

    let result = 0;
    println!("sum of top 3 : {}", result);

    return 0;
}

fn read_data(file_name:&str, rucksacks: &mut Vec<Rucksack>)
{
    let contents = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");

    let rucksack  = Rucksack
    {
        left_compartment : "".to_string(),
        right_compartment : "".to_string(),
    };
}

fn calculate_priority(item: &char) -> u32
{
    let priority = item.to_digit(36).expect("Invalid digit") - 9;

    return match item.is_lowercase()
    {
        true => priority,
        false => priority + 26,
    };
}
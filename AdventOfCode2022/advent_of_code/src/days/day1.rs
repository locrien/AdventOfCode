use std::fs;

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_part_1_test()
    {
        let expected = 24000;
        let result = run_part_1("assets/day1_test.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_1()
    {
        let expected = 70116;
        let result = run_part_1("assets/day1.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_2_test()
    {
        let expected = 45000;
        let result = run_part_2("assets/day1_test.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_2()
    {
        let expected = 206582;
        let result = run_part_2("assets/day1.txt");
        assert_eq!(result,expected);
    }
}

pub fn run_part_1(file_name:&str) -> u32
{
    let mut calories : Vec<u32> = Vec::new();
    read_calories(file_name, &mut calories);
    
    let result = *calories.last().expect("No elements found");
    println!("Elf with most calories transported : {}", result);

    return result;
}

pub fn run_part_2(file_name:&str) -> u32
{
    let mut calories : Vec<u32> = Vec::new();
    read_calories(file_name, &mut calories);

    let result = calories[&calories.len()-3..].iter().sum::<u32>();
    println!("sum of top 3 : {}", result);

    return result;
}

fn read_calories(file_name:&str, calories_list: &mut Vec<u32>)
{
    calories_list.push(0);

    let contents = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");

    for line in contents.lines() 
    {
        let last_idx = calories_list.len();

        match line.trim().parse::<u32>() 
        {
            Ok(num) => {calories_list[last_idx-1] += num},
            Err(_why) => calories_list.push(0),
        }
    }

    calories_list.sort();
}
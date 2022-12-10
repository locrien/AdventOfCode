use std::{fs, ops::Range};

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_part_1_test()
    {
        let expected = 2;
        let result = run_part_1("assets/day4_test.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_1()
    {
        let expected = 503;
        let result = run_part_1("assets/day4.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_2_test()
    {
        let expected = 4;
        let result = run_part_2("assets/day4_test.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_2()
    {
        let expected = 827;
        let result = run_part_2("assets/day4.txt");
        assert_eq!(result,expected);
    }
}


pub fn run_part_1(file_name:&str) -> u32
{
    let data = read_data(file_name);

    let mut amount = 0;
    for pair in data
    {
        if (pair.0.start <= pair.1.start && pair.0.end >= pair.1.end) || 
            (pair.0.start >= pair.1.start && pair.0.end <= pair.1.end)
        {
            amount += 1;
        }
    }

    println!("{}", amount);
    return amount;
}

pub fn run_part_2(file_name:&str) -> u32
{
    let data = read_data(file_name);

    let mut amount = 0;
    for pair in data
    {
        if (pair.0.start >= pair.1.start && pair.0.start <= pair.1.end) || 
            (pair.0.end >= pair.1.start && pair.0.end <= pair.1.end) ||
            (pair.1.start >= pair.0.start && pair.1.start <= pair.0.end) || 
            (pair.1.end >= pair.0.start && pair.1.end <= pair.0.end)
        {
            amount += 1;
        }
    }

    println!("{}", amount);
    return amount;
}

fn read_data(file_name:&str) -> Vec<(Range<u32>,Range<u32>)>
{
    let contents = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");

    return contents.lines().map(|x| read_pair(x)).collect();    
}

fn read_pair(pair_line:&str) -> (Range<u32>,Range<u32>)
{
    let n = pair_line.split(",").collect::<Vec<&str>>();
    let l = n[0].split("-").collect::<Vec<&str>>();
    let r = n[1].split("-").collect::<Vec<&str>>();

    return (l[0].parse().unwrap()..l[1].parse::<u32>().unwrap(),r[0].parse().unwrap()..r[1].parse::<u32>().unwrap());
}
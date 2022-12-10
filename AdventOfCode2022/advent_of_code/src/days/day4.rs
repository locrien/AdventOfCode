use std::ops::Range;

#[cfg(test)]
mod tests
{
    use super::*;
    use std::fs;
    use const_format::formatcp;

    const DAY: u32= 4;
    const FILE_NAME: &str = formatcp!("assets/day{DAY}.txt");
    const TEST_FILE_NAME: &str = formatcp!("assets/tests/day{DAY}.txt");
    #[test]
    fn part_1_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 2;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_1()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 503;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 4;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 827;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }
}

pub fn run(input:String, part: u32)
{
    match part
    {
        1 => _ = run_part_1(input),
        2 => _ = run_part_2(input),
        _ => println!("Invalid part"),
    }
}

fn run_part_1(input:String) -> u32
{
    let data = read_data(input);

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

fn run_part_2(input:String) -> u32
{
    let data = read_data(input);

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

fn read_data(input:String) -> Vec<(Range<u32>,Range<u32>)>
{
    return input.lines().map(|x| read_pair(x)).collect();    
}

fn read_pair(pair_line:&str) -> (Range<u32>,Range<u32>)
{
    let n = pair_line.split(",").collect::<Vec<&str>>();
    let l = n[0].split("-").collect::<Vec<&str>>();
    let r = n[1].split("-").collect::<Vec<&str>>();

    return (l[0].parse().unwrap()..l[1].parse::<u32>().unwrap(),r[0].parse().unwrap()..r[1].parse::<u32>().unwrap());
}
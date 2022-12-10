use std::collections::HashSet;

#[cfg(test)]
mod tests
{
    use super::*;
    use const_format::formatcp;
    use std::fs;

    const DAY: u32= 3;
    const FILE_NAME: &str = formatcp!("assets/day{DAY}.txt");
    const TEST_FILE_NAME: &str = formatcp!("assets/tests/day{DAY}.txt");

    #[test]
    fn part_1_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 157;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_1()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 8018;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 70;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 2518;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }
}

struct Rucksack
{
    pub all : String,
    pub left : String,
    pub right: String,
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
    let result =  read_data(input).iter()
        .map(|sack| { find_error(sack) })
        .map(|error| {calculate_priority(&error) }).sum();
    
    println!("{}", result);

    return result;
}

fn run_part_2(input:String) -> u32
{
    let result:u32 = read_data(input).chunks(3)
        .map(|g| {find_common_item(&g[0],&g[1],&g[2])})
        .map(|common| {calculate_priority(&common) }).sum();

    println!("{}", result);

    return result;
}

fn read_data(input:String) -> Vec<Rucksack>
{
    return input.lines().map(line_to_rucksack).collect();
}

fn line_to_rucksack(line:&str) -> Rucksack
{
    let split_line = line.split_at(line.len()/2);

    return Rucksack
    {
        left : split_line.0.to_string(),
        right :split_line.1.to_string(),
        all : line.to_string(),
    };
}

fn find_error(rucksack:&Rucksack) -> char
{
    return rucksack.left.chars().find(|x| {rucksack.right.contains(*x) }).expect("no errors found");
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

fn find_common_item(g1:&Rucksack,g2:&Rucksack,g3:&Rucksack) -> char
{
    let h1: HashSet<char> = HashSet::from_iter(g1.all.chars());
    let h2: HashSet<char> = HashSet::from_iter(g2.all.chars());
    let h3: HashSet<char> = HashSet::from_iter(g3.all.chars());

    let result: Vec<char> = h1.intersection(&h2).cloned().collect::<HashSet<char>>()
        .intersection(&h3).cloned().collect();

    return *result.get(0).expect("no common found");
}
use std::{fs, collections::HashSet};

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
        let expected = 8018;
        let result = run_part_1("assets/day3.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_2_test()
    {
        let expected = 70;
        let result = run_part_2("assets/day3_test.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_2()
    {
        let expected = 2518;
        let result = run_part_2("assets/day3.txt");
        assert_eq!(result,expected);
    }
}

struct Rucksack
{
    pub all : String,
    pub left : String,
    pub right: String,
}

pub fn run_part_1(file_name:&str) -> u32
{
    let result =  read_data(file_name).iter()
        .map(|sack| { find_error(sack) })
        .map(|error| {calculate_priority(&error) }).sum();
    
    println!("{}", result);

    return result;
}

pub fn run_part_2(file_name:&str) -> u32
{
    let result:u32 = read_data(file_name).chunks(3)
        .map(|g| {find_common_item(&g[0],&g[1],&g[2])})
        .map(|common| {calculate_priority(&common) }).sum();

    println!("{}", result);

    return result;
}

fn read_data(file_name:&str) -> Vec<Rucksack>
{
    let contents = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");

    return contents.lines().map(line_to_rucksack).collect();
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
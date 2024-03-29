use std::collections::HashMap;
use regex::Regex;

#[cfg(test)]
mod tests
{
    use super::*;
    use std::fs;
    use const_format::formatcp;

    const DAY: u32= 5;
    const FILE_NAME: &str = formatcp!("assets/day{DAY}.txt");
    const TEST_FILE_NAME: &str = formatcp!("assets/tests/day{DAY}.txt");
    #[test]
    fn part_1_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = "CMZ";
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_1()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = "CWMTGHBDW";
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = "MCD";
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = "SSCGWJCRB";
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }
}

// ----------------------------
struct Instruction
{
    pub amount:u32,
    pub from:u32,
    pub to:u32,
}

pub fn run_part_1(input:String) -> String
{
    let mut crate_stacks:HashMap<u32,Vec<char>> = HashMap::<u32, Vec<char>>::new();
    let mut instructions:Vec<Instruction> = Vec::new();
    read_data(input, &mut crate_stacks, &mut instructions);

    for instruction in instructions
    {
        apply_instruction(instruction, &mut crate_stacks, false);
    }

    let mut result:String = String::from("");
    for i in 1..crate_stacks.len()+1
    {
        let top_crate = crate_stacks.get(&(i as u32)).unwrap().last().unwrap();
        result.push(*top_crate);
    }
    println!("{}", result);
    return result;
}

pub fn run_part_2(input:String) -> String
{
    let mut crate_stacks:HashMap<u32,Vec<char>> = HashMap::<u32, Vec<char>>::new();
    let mut instructions:Vec<Instruction> = Vec::new();
    read_data(input, &mut crate_stacks, &mut instructions);

    for instruction in instructions
    {
        apply_instruction(instruction, &mut crate_stacks, true);
    }

    let mut result:String = String::from("");
    for i in 1..crate_stacks.len()+1
    {
        let top_crate = crate_stacks.get(&(i as u32)).unwrap().last().unwrap();
        result.push(*top_crate);
    }
    println!("{}", result);
    return result;
}

fn apply_instruction(instr:Instruction, stacks: &mut HashMap<u32,Vec<char>>, is_part_2: bool)
{
    if is_part_2
    {
        let mut to_move:Vec<char> = Vec::<char>::new();
        for _i in 1..instr.amount+1
        {
            let crate_stack = stacks.get_mut(&instr.from).unwrap();
            if crate_stack.len() > 0
            {
                let crate_to_move = crate_stack.pop().unwrap();
                to_move.push(crate_to_move);
            }
        }

        let crate_stack = stacks.get_mut(&instr.to).unwrap();
        for elem in to_move.iter().rev()
        {
            crate_stack.push(*elem);
        }
    } 
    else 
    {
        for _i in 1..instr.amount+1
        {
            let crate_stack = stacks.get_mut(&instr.from).unwrap();
            if crate_stack.len() > 0
            {
                let crate_to_move = crate_stack.pop().unwrap();
                let crate_stack = stacks.get_mut(&instr.to).unwrap();
                crate_stack.push(crate_to_move);
            }
        }
    }
}

fn read_data(input:String, starting_state: &mut HashMap<u32,Vec<char>>, instructions: &mut Vec<Instruction>)
{
    let split_content= input.split("\r\n\r\n").collect::<Vec<&str>>();

    let mut rev_lines = split_content[0].lines().collect::<Vec<&str>>();
    let spots:&str = rev_lines.pop().expect("msg");
    let re = Regex::new(r" (\d+) ").expect("error creating regex");
    let locs = re.captures_iter(spots);

    for loc in locs
    {
        let entry = loc.get(1).unwrap();
        let k:u32 = entry.as_str().parse().expect("");

        let mut v:Vec<char> = vec![];
        for crate_line in rev_lines.iter().rev()
        {
            let line =  *crate_line;
            let letter = line.get(entry.start()..entry.end()).expect("msg").chars().last().expect("msg");
            if letter.is_alphabetic()
            {
                v.push(letter);
            }
        }

        starting_state.insert(k, v);
    }

    //format : move 1 from 2 to 1
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").expect("error creating regex");
    instructions.append(&mut split_content[1].lines().map(|line| line_to_instruction(line, &re)).collect::<Vec<Instruction>>());
}

fn line_to_instruction(line:&str, re:&Regex) -> Instruction
{
    let captures = re.captures(line).unwrap();

    let instruction:Instruction = Instruction {
        from:captures.get(2).unwrap().as_str().parse().unwrap(), 
        to:captures.get(3).unwrap().as_str().parse().unwrap(), 
        amount:captures.get(1).unwrap().as_str().parse().unwrap()
    };

    return instruction;
}
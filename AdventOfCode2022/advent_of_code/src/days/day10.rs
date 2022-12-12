use std::collections::VecDeque;

#[cfg(test)]
mod tests
{
    use super::*;
    use std::fs;
    use const_format::formatcp;

    const DAY: u32= 10;
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

// ----------------------------

enum InstructionType
{
    Noop,
    Addx(i32)
}

struct Instruction
{
    pub instr_type:InstructionType,
    pub cycles:i32,
}

pub fn run_part_1(_input:String) -> u32
{
    let mut x = 1;
    let mut cycle = 1;
    let mut pending_ops:Vec<Instruction> = Vec::new();
    // add new instructions

    // process cycle


    

    return 0;
}

pub fn run_part_2(_input:String) -> u32
{
    return 0;
}

fn process_data(input: String) -> Result<Vec<Instruction>>
{
    return input.lines().map(|l| {  })
}

fn line_to_instruction(line:&str) -> Result<Instruction,Err>
{
    if line.starts_with("noop")
    {
        return Ok(Instruction 
            {
                cycles : 1,
                instr_type : InstructionType::Noop,
            });
    }
    else if line.starts_with("addx")
    {
        let amount = line.split(" ").last().unwrap().parse().unwrap();
        return Ok(Instruction 
            {
                cycles : 2,
                instr_type : InstructionType::Addx(amount),
            });
    }
    else 
    {
        return Err;
    }
}
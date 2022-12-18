use std::fmt;

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

        let expected = 13140;
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

        let expected = 0;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 0;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }
}

// ----------------------------

#[derive(Debug)]
enum InstructionType
{
    Invalid,
    Noop,
    Addx(i32)
}

impl fmt::Display for InstructionType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct Instruction
{
    pub instr_type:InstructionType,
    pub cycles:i32,
}

pub fn run_part_1(input:String) -> i32
{
    let instructions = process_data(input);

    let mut x = 1;
    let mut cycle = 0;
    let mut signal_strength = 0;


    println!("");

    for instruction in &instructions
    {
        for _ in 0..instruction.cycles
        {
            cycle = cycle + 1;

            if (cycle + 20) % 40 == 0
            {
                signal_strength = signal_strength + (cycle * x);
                println!("signal strength (c {},x {}) : {}",cycle,x,signal_strength);
            }
        }

        match instruction.instr_type
        {
            InstructionType::Addx(val) => 
            {
                x = x + val; 
            },
            _ => {},
        }
    }    

    return signal_strength;
}

pub fn run_part_2(input:String) -> u32
{
    let instructions = process_data(input);

    let mut x = 1;
    let mut cycle = 0;

    let mut screen_content = Vec::<char>::new();

    for instruction in &instructions
    {
        for _ in 0..instruction.cycles
        {
            cycle = cycle + 1;

            // check if x is within 1 of cycle
            if (cycle - 1)%40 >= x - 1 && (cycle - 1)%40 <= x + 1
            {
                screen_content.push('#');
            }
            else
            {
                screen_content.push('.');
            }
        }

        match instruction.instr_type
        {
            InstructionType::Addx(val) => 
            {
                x = x + val; 
            },
            _ => {},
        }
    }

    let result = String::from_iter(screen_content.iter());

    let mut i = 0;
    while i + 40 <= result.len()
    {
        println!("{}", result.get(i..i+40).expect("msg"));
        i = i + 40;
    }

    return 0;
}

fn process_data(input: String) -> Vec<Instruction>
{
    return input.lines().map(|l| line_to_instruction(l)).collect();
}

fn line_to_instruction(line:&str) -> Instruction
{
    if line.starts_with("noop")
    {
        return Instruction 
            {
                cycles : 1,
                instr_type : InstructionType::Noop,
            };
    }
    else if line.starts_with("addx")
    {
        let amount = line.split(" ").last().unwrap().parse().unwrap();
        return Instruction 
            {
                cycles : 2,
                instr_type : InstructionType::Addx(amount),
            };
    }
    else 
    {
        return Instruction
        {
            cycles : 1,
            instr_type : InstructionType::Invalid,
        };
    }
}
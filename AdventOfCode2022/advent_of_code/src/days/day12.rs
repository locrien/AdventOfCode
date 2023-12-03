#[cfg(test)]
mod tests
{
    use super::*;
    use std::fs;
    use const_format::formatcp;

    const DAY: u32= 12;
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
pub fn run_part_1(input:String) -> u32
{
    let world = parse_input(input);
    let mut start_pos = (0,0);
    
    'line: for l in 0..world.len()
    {
        for s in 0..world[l].len()
        {
            match world[l][s]
            {
                SpaceType::Start(_) => 
                {
                    start_pos = (l,s);
                    break 'line;
                }
                _ => {}
            }            
        }
    }

    // find path

    return 0;
}

pub fn run_part_2(_input:String) -> u32
{
    return 0;
}

#[derive(PartialEq)]
enum SpaceType
{
    Start(u32),
    Normal(u32),
    End(u32),
    Invalid,
}

fn parse_input(input:String) -> Vec<Vec<SpaceType>>
{
    return input.lines().map(|l| { l.chars().map(|c| { map_char_to_int(c) }).collect::<Vec<SpaceType>>() }).collect::<Vec<Vec<SpaceType>>>();
}

fn map_char_to_int(c:char) -> SpaceType
{
    return match c
    {
        'S' => SpaceType::Start(0),
        'a' => SpaceType::Normal(0),
        'b' => SpaceType::Normal(1),
        'c' => SpaceType::Normal(2),
        'd' => SpaceType::Normal(3),
        'e' => SpaceType::Normal(4),
        'f' => SpaceType::Normal(5),
        'g' => SpaceType::Normal(6),
        'h' => SpaceType::Normal(7),
        'i' => SpaceType::Normal(8),
        'j' => SpaceType::Normal(9),
        'k' => SpaceType::Normal(10),
        'l' => SpaceType::Normal(11),
        'm' => SpaceType::Normal(12),
        'n' => SpaceType::Normal(13),
        'o' => SpaceType::Normal(14),
        'p' => SpaceType::Normal(15),
        'q' => SpaceType::Normal(16),
        'r' => SpaceType::Normal(17),
        's' => SpaceType::Normal(18),
        't' => SpaceType::Normal(19),
        'u' => SpaceType::Normal(20),
        'v' => SpaceType::Normal(21),
        'w' => SpaceType::Normal(22),
        'x' => SpaceType::Normal(23),
        'y' => SpaceType::Normal(24),
        'z' => SpaceType::Normal(25),
        'E' => SpaceType::End(0),
        _ => SpaceType::Invalid,
    };
}
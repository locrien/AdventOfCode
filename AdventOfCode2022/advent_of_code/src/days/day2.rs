use std::fs;

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_part_1_test()
    {
        let expected = 15;
        let result = run_part_1("assets/day2_test.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_1()
    {
        let expected = 11767;
        let result = run_part_1("assets/day2.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_2_test()
    {
        let expected = 12;
        let result = run_part_2("assets/day2_test.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_2()
    {
        let expected = 13886;
        let result = run_part_2("assets/day2.txt");
        assert_eq!(result,expected);
    }
}

enum Choice
{
    Rock,
    Paper,
    Scissors,
    Invalid,
}

pub fn run_part_1(file_name:&str) -> u32
{
    let result = read_data(file_name,false);
    let total_score:u32 = result.iter().map(|round| calculate_score(round)).sum();
    println!("score : {}", total_score);

    return total_score;
}

pub fn run_part_2(file_name:&str) -> u32
{
    let result = read_data(file_name,true);
    let total_score:u32 = result.iter().map(|round| calculate_score(round)).sum();
    println!("score : {}", total_score);

    return total_score;
}

fn read_data(file_name:&str, is_part_2:bool) -> Vec<(Choice,Choice)>
{
    let contents = fs::read_to_string(file_name)
    .expect("Should have been able to read the file");

    let content_lines =  contents.lines();
    let result:Vec<(Choice,Choice)> = content_lines.map(|line| transform_line(line,is_part_2)).collect();
    return result;
}

fn transform_line(line: &str,is_part_2:bool) -> (Choice, Choice)
{
    let split_result = line.split(' ').collect::<Vec<&str>>();
    if is_part_2
    {
        let other_input = map_to_choice(split_result[0]);
        let my_input = map_to_choice2(split_result[1],&other_input);
        return (other_input,my_input);
    }
    else
    {
        return (map_to_choice(split_result[0]),map_to_choice(split_result[1]));
    }
}

fn map_to_choice(input: &str) -> Choice
{
    return match input
    {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissors,
        _ => Choice::Invalid,
    };
}

fn map_to_choice2(input: &str, other_choice: &Choice) -> Choice
{
    return match input
    {
        "X" => match other_choice 
        {
            Choice::Paper => Choice::Rock,
            Choice::Rock => Choice::Scissors,
            Choice::Scissors => Choice::Paper,
            _ => Choice::Invalid,
        },
        "Y" => match other_choice 
        {
            Choice::Paper => Choice::Paper,
            Choice::Rock => Choice::Rock,
            Choice::Scissors => Choice::Scissors,
            _ => Choice::Invalid,
        },
        "Z" => match other_choice 
        {
            Choice::Paper => Choice::Scissors,
            Choice::Rock => Choice::Paper,
            Choice::Scissors => Choice::Rock,
            _ => Choice::Invalid,
        },
        _ => Choice::Invalid,
    };
}

fn calculate_score(round:&(Choice,Choice)) -> u32
{
    let loss_points:u32 = 0;
    let draw_points:u32 = 3;
    let victory_points:u32 = 6;
    let rock_bonus:u32 = 1;
    let paper_bonus:u32 = 2;
    let scissors_bonus:u32 = 3;

    return match round
    {
        (Choice::Paper, Choice::Rock) => loss_points + rock_bonus,
        (Choice::Scissors, Choice::Paper) => loss_points + paper_bonus,
        (Choice::Rock, Choice::Scissors) => loss_points + scissors_bonus,
        (Choice::Rock, Choice::Rock) => draw_points + rock_bonus,
        (Choice::Paper, Choice::Paper) => draw_points + paper_bonus,
        (Choice::Scissors, Choice::Scissors) => draw_points + scissors_bonus,
        (Choice::Scissors, Choice::Rock) => victory_points + rock_bonus,
        (Choice::Rock, Choice::Paper) => victory_points + paper_bonus,
        (Choice::Paper, Choice::Scissors) => victory_points + scissors_bonus,
        _ => 0,
    }
}
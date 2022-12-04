use std::fs;

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_part_1_test()
    {
        let expected = 2;
        let result = run_part_1("assets/day3_test.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_1()
    {
        let expected = 0;
        let result = run_part_1("assets/day3.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_2_test()
    {
        let expected = 0;
        let result = run_part_2("assets/day3_test.txt");
        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_2()
    {
        let expected = 0;
        let result = run_part_2("assets/day3.txt");
        assert_eq!(result,expected);
    }
}


pub fn run_part_1(file_name:&str) -> u32
{
    read_data(file_name);

    return 0;
    
}

pub fn run_part_2(file_name:&str) -> u32
{
    read_data(file_name);
    return 0;
}

fn read_data(file_name:&str)
{
    let contents = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");

}
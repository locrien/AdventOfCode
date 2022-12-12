#[cfg(test)]
mod tests
{
    use super::*;
    use std::fs;
    use const_format::formatcp;

    const DAY: u32= 7;
    const FILE_NAME: &str = formatcp!("assets/day{DAY}.txt");
    const TEST_FILE_NAME: &str = formatcp!("assets/tests/day{DAY}.txt");
    #[test]
    fn part_1_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 95437;
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

struct Directory<'a>
{
    pub name:&'static str,
    parent:&'a  Directory<'a>,
    sub_content:Vec<&'a dyn Content>,
}

struct File<'a>
{
    pub name:&'static str,
    parent:&'a  Directory<'a>,
    size:u32,
}

trait Content
{
    fn name(&self) -> &'static str;

    fn Size(&self) -> u32;
    fn GetParent(&self) -> &Directory;
}

impl Content for Directory<'_>
{
    fn name(&self) -> &'static str {
        self.name
    }

    fn Size(&self) -> u32
    {
        return 0;
    }

    fn GetParent(&self) -> &Directory {
        return self.parent;
    }
}

impl Content for File<'_>
{
    fn name(&self) -> &'static str {
        self.name
    }

    fn Size(&self) -> u32
    {
        return self.size;
    }

    fn GetParent(&self) -> &Directory {
        return self.parent;
    }
}

// ----------------------------
pub fn run_part_1(_input:String) -> u32
{
    /*let root_directory = Directory {
        name : "test1",
        sub_content : Vec::<&dyn Content>::new(),
    };

    let file1 = File {
        name : "",
        size : 100,
        parent : &root_directory,
    };

    let dir1 = Directory {
        name : "test2",
        sub_content : vec![&file1],
    };

    let mut current_directory = root_directory;

    println!("{}", current_directory.Size());

    current_directory = dir1;

    println!("{}", current_directory.Size());*/
    
    return 0;
}

pub fn run_part_2(_input:String) -> u32
{
    return 0;
}
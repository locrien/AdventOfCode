#[cfg(test)]
mod tests
{
    use super::*;
    use std::fs;
    use const_format::formatcp;

    const DAY: u32= 14;
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
// sand falling problem - https://adventofcode.com/2022/day/14

/*enum SpaceState
{
    Source,
    Rock,
    Air,
    Sand,
}
struct World
{
    sand_in_pos:(i32,i32),
    origin_pos:(i32,i32),
    lowest_rock_pos:i32,
    spaces:VecDeque<VecDeque<SpaceState>>,
}*/

pub fn run_part_1(_input:String) -> u32
{
    //let world = process_input(_input);
    return 0;
}

pub fn run_part_2(_input:String) -> u32
{
    return 0;
}

/*fn process_input(input:String) -> World
{
    let mut world = World
    {
        sand_in_pos : (500,0),
        spaces: VecDeque::<VecDeque<SpaceState>>::new(),
        origin_pos: (500,0),
        lowest_rock_pos: -1,
    };
    world.spaces.push_back(VecDeque::<SpaceState>::new());

    insert_at_position(&mut world, (500,0), SpaceState::Source);

    input.lines().map(|l| {  });

    return world;
}*/

/*fn insert_at_position(world:&mut World, pos:(i32,i32), insert_type:SpaceState)
{
    //let index:(usize,usize) = ((pos.0 - world.origin_pos.0) as usize, (pos.1 - world.origin_pos.1) as usize);

    /*while pos.0 < world.origin_pos.0
    {
        world.spaces.push_front(VecDeque::<SpaceState>::new());
        world.origin_pos.0
    }

    while index.0 < world.spaces.len()
    {
        world.spaces.push_back(VecDeque::<SpaceState>::new());
    }

   
    world.spaces.push_back(VecDeque::<SpaceState>::new());
    world.spaces[0].push_front(SpaceState::Source);*/
}*/
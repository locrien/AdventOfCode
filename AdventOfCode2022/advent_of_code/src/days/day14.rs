use std::collections::HashMap;

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

        let expected = 24;
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

        let expected = 93;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 30762;
        let result = run_part_2(input);
        assert_eq!(result,expected);
    }
}

// ----------------------------
// sand falling problem - https://adventofcode.com/2022/day/14

enum SpaceState
{
    Rock,
    Sand,
}

#[derive(PartialEq)]
enum StepResult
{
    Moving,
    Resting,
    Dropped,
}

struct World
{
    sand_in_pos:(usize,usize),
    rock_borders:((usize,usize),(usize,usize)), //min x y, max x y
    occupied_spaces:HashMap<(usize,usize),SpaceState>,
    active_pos:(usize,usize),
}

impl World
{
    fn new(sand_in:(usize,usize)) -> Self
    {
        return World
        {
            rock_borders:(sand_in,sand_in),
            sand_in_pos:sand_in,
            occupied_spaces:HashMap::<(usize,usize),SpaceState>::new(),
            active_pos:sand_in,
        }
    }

    fn step(&mut self, part_two:bool) -> StepResult
    {
        if part_two && self.active_pos.1 >= self.rock_borders.1.1 + 1
        {
            return StepResult::Resting;
        }

        if !part_two && self.active_pos.1 > self.rock_borders.1.1
        {
            return StepResult::Dropped;
        }

        self.occupied_spaces.remove(&self.active_pos);
        
        // Check can go down
        let mut position = self.active_pos;
        position.1 = position.1 +1;

        let tile = self.occupied_spaces.get_mut(&position);
        match tile
        {
            None => 
            {
                // place 
                
                self.occupied_spaces.insert(position, SpaceState::Sand);
                self.active_pos = position;
                
                return StepResult::Moving;
            },
            _ => {},
        }

        // check can go down left
        let mut position = self.active_pos;
        position.0 = position.0 - 1;
        position.1 = position.1 +1;

        let tile = self.occupied_spaces.get_mut(&position);
        match tile
        {
            None => 
            {
                // place 
                self.occupied_spaces.insert(position, SpaceState::Sand);
                self.active_pos = position;
                return StepResult::Moving;
            },
            _ => {},
        }

        // check can go down right
        let mut position = self.active_pos;
        position.0 = position.0 +1;
        position.1 = position.1 +1;

        let tile = self.occupied_spaces.get_mut(&position);
        match tile
        {
            None => 
            {
                // place 
                self.occupied_spaces.insert(position, SpaceState::Sand);
                self.active_pos = position;
                return StepResult::Moving;
            },
            _ => {},
        }

        self.occupied_spaces.insert(self.active_pos, SpaceState::Sand);
        if part_two && self.active_pos.0 == self.sand_in_pos.0 && self.active_pos.1 == self.sand_in_pos.1
        {
            return StepResult::Dropped;
        }
        return StepResult::Resting;
    }

    fn spawn_sand(&mut self)
    {
        let can_spawn =  match self.occupied_spaces.get(&self.sand_in_pos)
        {
            None => { true },
            _ => { false },
        };

        if can_spawn
        {
            self.active_pos = self.sand_in_pos;
            self.occupied_spaces.insert(self.sand_in_pos, SpaceState::Sand);
        }
    }

    fn insert_rock(&mut self, pos:(usize,usize))
    {
        self.occupied_spaces.insert(pos, SpaceState::Rock);
        self.rock_borders.0.0 = if pos.0 < self.rock_borders.0.0 { pos.0 } else {self.rock_borders.0.0};
        self.rock_borders.0.1 = if pos.1 < self.rock_borders.0.1 { pos.1 } else {self.rock_borders.0.1};
        self.rock_borders.1.0 = if pos.0 > self.rock_borders.1.0 { pos.0 } else {self.rock_borders.1.0};
        self.rock_borders.1.1 = if pos.1 > self.rock_borders.1.1 { pos.1 } else {self.rock_borders.1.1};
    }

    #[allow(dead_code)]
    fn print(&self)
    {
        let min_x = self.rock_borders.0.0 as usize;
        let max_x = self.rock_borders.1.0 as usize;
        let min_y = self.rock_borders.0.1 as usize;
        let max_y = self.rock_borders.1.1 as usize;
        for y in min_y..max_y+1
        {
            println!("");
            for x in min_x..max_x+1
            {
                if self.sand_in_pos.0 == x && self.sand_in_pos.1 == y
                {
                    print!("+");
                    continue;
                }

                match self.occupied_spaces.get(&(x,y))
                {
                    None => {print!(".")}, // air
                    Some(val) => 
                    {
                        match val
                        {
                            SpaceState::Rock => { print!("#")},
                            SpaceState::Sand => {print!("o")},
                        }
                    }
                }
            }
        }
    }
}

pub fn run_part_1(input:String) -> u32
{
    let mut world = process_input(input);
    let mut step_result = StepResult::Moving;
    let mut rested_amount = 0;

    while step_result != StepResult::Dropped 
    {
        world.spawn_sand();
        step_result = world.step(false);

        while step_result == StepResult::Moving
        {
            step_result = world.step(false);
        }

        if step_result == StepResult::Resting
        {
            rested_amount = rested_amount + 1;
        }
    }
     
    return rested_amount;
}

pub fn run_part_2(input:String) -> u32
{
    let mut world = process_input(input);
    let mut step_result = StepResult::Moving;
    let mut rested_amount = 0;

    while step_result != StepResult::Dropped 
    {
        world.spawn_sand();
        step_result = world.step(true);

        while step_result == StepResult::Moving
        {
            step_result = world.step(true);
        }

        if step_result == StepResult::Resting
        {
            rested_amount = rested_amount + 1;
        }
    }
     
    return rested_amount+1;
}

fn process_input(input:String) -> World
{
    let mut world = World::new((500,0));

    for line in input.lines()
    {
        let positions:Vec<(usize,usize)> = line.split(" -> ").collect::<Vec<&str>>().iter().map(|s| { convert_to_pos(s)}).collect();
        for i in 1..positions.len()
        {
            let from = positions[i-1];
            let to = positions[i];

            let mut delta = (to.0 as i32 - from.0 as i32,to.1 as i32 - from.1 as i32);
            if delta.0 != 0
            {
                delta = (delta.0/delta.0*delta.0.signum(),delta.1);
            }

            if delta.1 != 0
            {
                delta =  (delta.0,delta.1/delta.1*delta.1.signum());
            }
            
            let mut insert_pos = from;

            while insert_pos != to 
            {
                world.insert_rock(insert_pos);
                insert_pos.0 = (insert_pos.0 as i32 + delta.0) as usize;
                insert_pos.1 = (insert_pos.1 as i32 + delta.1) as usize;
            }

            world.insert_rock(to);
        }
    }

    return world;
}

fn convert_to_pos(input:&str) -> (usize,usize)
{
    let sides = input.split(",").collect::<Vec<&str>>();
    let left_side:usize = sides[0].parse().unwrap();
    let right_side:usize = sides[1].parse().unwrap();

    return (left_side,right_side);
}
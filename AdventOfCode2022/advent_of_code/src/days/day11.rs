use std::collections::VecDeque;

#[cfg(test)]
mod tests
{
    use super::*;
    use std::fs;
    use const_format::formatcp;

    const DAY: u64= 11;
    const FILE_NAME: &str = formatcp!("assets/day{DAY}.txt");
    const TEST_FILE_NAME: &str = formatcp!("assets/tests/day{DAY}.txt");
    #[test]
    fn part_1_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 10605;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_1()
    {
        let input = fs::read_to_string(FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 56350;
        let result = run_part_1(input);
        assert_eq!(result,expected);
    }

    #[test]
    fn part_2_example()
    {
        let input = fs::read_to_string(TEST_FILE_NAME)
        .expect("Should have been able to read the file");

        let expected = 2713310158;
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


struct Monkey
{
    pub held_items:VecDeque<u64>,
    pub operation:String,
    pub div_test:u64,
    pub true_target:usize,
    pub false_target:usize,
    pub inspect_count:u64,
}

pub fn run_part_1(input:String) -> u64
{
    let mut monkeys = parse_input(input);

    // rounds
    for _ in 0..20
    {
        //println!("Round : {}", round);
        for m in 0..monkeys.len()
        {
            //println!("Monkey {}:", monkeys[m].id);
            while monkeys[m].held_items.len() > 0
            {
                monkeys[m].inspect_count = monkeys[m].inspect_count + 1;
                // inspect op
                let old_item = monkeys[m].held_items.pop_front().unwrap();
                //println!("Monkey inspects an item with a worry level of {}.", old_item);
                // apply op
                let new_item = perform_op(&monkeys[m].operation,old_item);
                //println!("Worry level is changed to {}.", new_item);
                // div by 3
                let new_item = new_item / 3;
                //println!("Monkey gets bored with item. Worry level is divided by 3 to {}.", new_item);
                // test
                let test_result = new_item % monkeys[m].div_test == 0;
               // println!("test divisible by {} result : {}", monkeys[m].div_test, test_result);
                let target_id:usize = if test_result { monkeys[m].true_target } else {monkeys[m].false_target} ;
                // throw
                monkeys[target_id].held_items.push_back(new_item);
                //println!(" Item with worry level {} is thrown to monkey {}.", new_item,target_id);
            }
        }
    }

    // multiply 2 most active monkey inspect_count

    let mut test = monkeys.iter().map(|m| { m.inspect_count }).collect::<Vec<u64>>();
    test.sort();
    test = test.drain(test.len()-2..).collect();
    return test[0] * test[1];
}

pub fn run_part_2(input:String) -> u64
{
    let mut monkeys = parse_input(input);

    let lcd: u64 = monkeys.iter().fold(1,|acc, m| { acc * m.div_test });
    // rounds
    for _ in 0..10000
    {
        //println!("Round : {}", round);
        for m in 0..monkeys.len()
        {
            //println!("Monkey {}:", monkeys[m].id);
            while monkeys[m].held_items.len() > 0
            {
                monkeys[m].inspect_count = monkeys[m].inspect_count + 1;
                // inspect op
                let old_item = monkeys[m].held_items.pop_front().unwrap();
                //println!("Monkey inspects an item with a worry level of {}.", old_item);
                // apply op##
                let new_item = perform_op(&monkeys[m].operation,old_item);
                //println!("Worry level is changed to {}.", new_item);
                // div by 3
                //let new_item = new_item / 3;
                
                let new_item = new_item % lcd;
                //println!("Monkey gets bored with item. Worry level is divided by 3 to {}.", new_item);
                // test
                let test_result = new_item % monkeys[m].div_test == 0;

                // println!("test divisible by {} result : {}", monkeys[m].div_test, test_result);
                let target_id:usize = if test_result { monkeys[m].true_target } else {monkeys[m].false_target} ;

                // throw
                monkeys[target_id].held_items.push_back(new_item);
                //println!(" Item with worry level {} is thrown to monkey {}.", new_item,target_id);
            }
        }
    }

    // multiply 2 most active monkey inspect_count

    let mut test = monkeys.iter().map(|m| { m.inspect_count }).collect::<Vec<u64>>();
    test.sort();
    test = test.drain(test.len()-2..).collect();
    return test[0] * test[1];
}

fn parse_input(input: String) -> Vec<Monkey>
{
    /* format :
    
    Monkey 0:
    Starting items: 79, 98

    Operation: new = old * 19
    Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
    
    */
    let monkey_inputs:Vec<&str> = input.split("\r\n\r\n").collect();

    let mut monkeys = Vec::<Monkey>::new();
    for monkey_input in monkey_inputs
    {
        let mut lines = monkey_input.lines();

        let items_input = lines.next().unwrap().strip_prefix("  Starting items: ").unwrap();
        let held_items = items_input.split(',').map(|t| t.trim().parse().unwrap()).collect();
        let monkey = Monkey
        {
            held_items,
            operation: String::from(lines.next().unwrap().strip_prefix("  Operation: new = ").unwrap()),
            div_test:lines.next().unwrap().strip_prefix("  Test: divisible by ").unwrap().trim().parse().unwrap(),
            true_target:lines.next().unwrap().strip_prefix("    If true: throw to monkey ").unwrap().trim().parse().unwrap(),
            false_target:lines.next().unwrap().strip_prefix("    If false: throw to monkey ").unwrap().trim().parse().unwrap(),
            inspect_count: 0,
        };

        monkeys.push(monkey);
    }

    return monkeys;
}

fn perform_op(input:&String, old:u64) -> u64
{
    let alt_input = input.replace("old", old.to_string().as_str());
    let formula = alt_input.split(' ').collect::<Vec<&str>>();
    
    let l = formula[0].trim().parse::<u64>().unwrap();
    let r = formula[2].trim().parse::<u64>().unwrap();
    return match formula[1]
    {
        "+" => l + r,
        "*" => l * r,
        "-" => l - r,
        "/" => l / r,
        _ => 0,
    };
}
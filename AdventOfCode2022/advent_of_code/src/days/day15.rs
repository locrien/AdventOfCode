#[cfg(test)]
mod tests
{
    use super::*;
    use std::fs;
    use const_format::formatcp;

    const DAY: u32= 15;
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
pub fn run_part_1(_input:String) -> u32
{
    main();
    return 0;
}

pub fn run_part_2(_input:String) -> u32
{
    return 0;
}

use regex::Regex;
use std::collections::VecDeque;

//https://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-a-struct-in-rust
struct Monkey<F> 
where F: Fn(i32) -> i32, {
	items: VecDeque<i32>,
	test: i32,
	if_true: usize,
	if_false: usize,
	op: F,
}

enum Value {
	Integer(i32),
	Old,
}

impl Value {
	fn eval(&self, old:&i32) -> i32 {
		match *self {
			Value::Old => *old,
			Value::Integer(i) => i
		}
	}
}

fn get_value(v: &str) -> Value {
	match v {
		"old" => Value::Old,
		_ => Value::Integer(v.parse().unwrap())
	}
}

fn get_op(op: &char) -> Box<dyn Fn(i32, i32) -> i32> {
	match op {
		'+' => Box::new(move |a: i32 , b:i32| a + b),
		'-' => Box::new(move |a: i32 , b:i32| a - b),
		'*' => Box::new(move |a: i32 , b:i32| a * b),
		'/' => Box::new(move |a: i32 , b:i32| a / b),
		_ => unreachable!()
	}
}

fn parse_op(input: &str) -> Box<dyn Fn(i32) -> i32> {
	let parts: Vec<&str> = input.split(' ').collect();
	let a = get_value(parts[0]);
	let b = get_value(parts[2]);
	let op = get_op(&parts[1].chars().next().unwrap());
	Box::new(move |v: i32| op(a.eval(&v), b.eval(&v)))
}

fn main() 
{
	let contents = std::fs::read_to_string("testinput.txt").expect("Can't read the file");
	let re = Regex::new(&std::fs::read_to_string("re.txt").unwrap()).expect("Invalid regex");

	let mut monkeys = Vec::new();
	for cap in re.captures_iter(&contents) {
		let m = Monkey {
			items : cap[2].split(", ").map(|s| s.parse().unwrap()).collect(),
			test : cap[4].parse().unwrap(),
			if_true  : cap[5].parse().unwrap(),
			if_false : cap[6].parse().unwrap(),
			op: parse_op(&cap[3]),
		};

		monkeys.push(m);
		//println!("TEST {}", (m.op)(2));
	}

	// round
    let mut added_items:Vec<(usize,i32)> = Vec::<(usize,i32)>::new();
	for m in &mut monkeys {
		while !m.items.is_empty() {
			let item = m.items.pop_front().unwrap();
			let mut newitem = (m.op)(item);
			newitem /= 3;
			println!("item {item}->{newitem}");
			let target = if newitem % m.test == 0 { m.if_true } else { m.if_false };
			//let tm = monkeys.get_mut(target).unwrap(); // < second mutable borrow fucking what
			//tm.items.push_back(newitem);
            added_items.push((target, newitem));
		}
	}

    for added_item in added_items
    {
        let tm = monkeys.get_mut(added_item.0).unwrap(); // < second mutable borrow fucking what
		tm.items.push_back(added_item.1);
    }


}


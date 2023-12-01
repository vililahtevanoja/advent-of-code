#![feature(test)]

use std::cell::RefCell;

extern crate test;

#[derive(Debug)]
enum Operation {
    Add(i64),
    Multiply(i64),
    Square,
    Double,
}

impl Operation {
    fn apply(&self, input: i64) -> i64 {
        match self {
            Operation::Add(n) => input + n,
            Operation::Multiply(n) => input * n,
            Operation::Square => input * input,
            Operation::Double => input + input,
        }
    }
}

impl Operation {
    fn from(s: &str) -> Operation {
        assert!(s.trim().starts_with("Operation"));
        let op = s.split_once("old").unwrap().1.trim();
        let operand = op.split_once(" ").unwrap().1;
        if op.starts_with("*") {
            if operand == "old" {
                return Operation::Square;
            }
            let operand_n = operand.parse::<i64>().unwrap();
            return Operation::Multiply(operand_n);
        } else if op.starts_with("+") {
            if operand == "old" {
                return Operation::Double;
            }
            let operand_n = operand.parse::<i64>().unwrap();
            return Operation::Add(operand_n);
        } else {
            panic!("Weird op value {} from {}", op, s);
        }
    }
}

#[derive(Debug)]
struct Monkeys {
    count: usize,
    monkeys: Vec<Monkey>,
}

impl Monkeys {
    fn from(monkeys: Vec<Monkey>) -> Monkeys {
        Monkeys {
            count: monkeys.len(),
            monkeys: monkeys,
        }
    }
    fn push_item_to(&mut self, i: usize, item: i64) {
        self.monkeys[i].items.borrow_mut().push(item);
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: RefCell<Vec<i64>>,
    operation: Operation,
    test: i64,
    test_true: usize,
    test_false: usize,
}

impl Monkey {
    fn from(s: &str) -> Monkey {
        let mut lines = s.lines();
        let monkey_id = lines
            .next()
            .unwrap()
            .split_once(" ")
            .unwrap()
            .1
            .trim_end_matches(":")
            .parse::<usize>()
            .unwrap();
        let starting_items = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|e| e.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let operation = Operation::from(lines.next().unwrap());
        let test = lines
            .next()
            .unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .parse::<i64>()
            .unwrap();
        let test_true = lines
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let test_false = lines
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        Monkey {
            id: monkey_id,
            items: RefCell::new(starting_items),
            operation: operation,
            test: test,
            test_true: test_true,
            test_false: test_false,
        }
    }

    fn push_item(&mut self, item: i64) {
        self.items.borrow_mut().push(item);
    }
}

fn parse(data: &str) -> Monkeys {
    let monkeys = data
        .split("\n\n")
        .map(Monkey::from)
        .collect::<Vec<Monkey>>();
    Monkeys::from(monkeys)
}

fn solve1(data: &str) -> usize {
    let mut monkeys: Monkeys = parse(data);
    let num_monkeys = monkeys.count;
    let mut monkeyInspections = vec![0usize; num_monkeys];
    println!("{:?}", monkeys);
    for _ in 0..1 {
        for i in 0..num_monkeys {
            let mut monkey = monkeys[i];
            let getPrevInspections = monkeyInspections.get_mut(i).unwrap();
            monkeyInspections[i] = *getPrevInspections + monkey.items.borrow().len();
            for item in monkey.items.borrow().iter() {
                let mut worry_level = item.clone();
                worry_level = monkey.operation.apply(worry_level);
                let throw_to = if worry_level % monkey.test == 0 {
                    monkey.test_true
                } else {
                    monkey.test_false
                };
                monkeys.push_item_to(i, worry_level);
            }
        }
    }
    0
}

fn solve2(data: &str) -> usize {
    0
}

fn main() {
    let data = include_str!("../input.txt");
    println!("Part 1: {}", solve1(data));
    println!("Part 2: {}", solve2(data));
}

mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        let data = include_str!("../example.txt");
        assert_eq!(solve1(data), 10605)
    }

    #[test]
    fn test_part1_regression() {
        let data = include_str!("../input.txt");
        assert_eq!(solve1(data), 6236)
    }

    #[test]
    fn test_part2() {
        let data1 = include_str!("../example.txt");
        assert_eq!(solve2(data1), 1);
    }

    #[test]
    fn test_part2_regression() {
        let data = include_str!("../input.txt");
        assert_eq!(solve2(data), 2449)
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let data = include_str!("../input.txt");
        b.iter(|| solve1(data));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let data = include_str!("../input.txt");
        b.iter(|| solve2(data));
    }
}

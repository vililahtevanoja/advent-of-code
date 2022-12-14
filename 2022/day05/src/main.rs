#![feature(test)]
extern crate test;

use regex::Regex;

#[derive(Debug)]
struct Move {
    count: u8,
    from: usize,
    to: usize,
}

impl Move {
    fn from(s: &str) -> Move {
        let move_regex: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        for cap in move_regex.captures_iter(s) {
            return Move {
                count: cap[1].parse::<u8>().unwrap(),
                from: cap[2].parse::<usize>().unwrap() - 1,
                to: cap[3].parse::<usize>().unwrap() - 1,
            };
        }
        panic!("Should not end here for {}", s);
    }
}

type Stack = Vec<char>;

fn parse_stacks(s: Vec<&str>) -> Vec<Stack> {
    let indices: Vec<usize> = s
        .first()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_numeric())
        .map(|(i, _)| i)
        .collect();
    let mut stacks = vec![Stack::new(); indices.len()];
    for l in s.iter().skip(1).map(|l| l.chars().collect::<Vec<char>>()) {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let str_i = indices.get(i).unwrap();
            let c = l.get(*str_i).unwrap();
            if c.is_alphabetic() {
                stack.push(*c);
            }
        }
    }
    stacks
}

fn parse(data: &str) -> (Vec<Move>, Vec<Stack>) {
    let (stacks, moves) = data.split_once("\n\n").unwrap();
    let mut stack_data_rev: Vec<&str> = stacks.split("\n").collect();
    stack_data_rev.reverse();
    let moves_vec = moves.lines().map(Move::from).collect();
    let parsed_stacks = parse_stacks(stack_data_rev);
    return (moves_vec, parsed_stacks);
}

fn do_moves_p1(moves: Vec<Move>, stacks: &mut Vec<Stack>) {
    for m in moves.iter() {
        let mut items = Vec::new();
        {
            let from = stacks.get_mut(m.from).unwrap();
            items = from.split_off(from.len() - (m.count as usize));
        }
        {
            let to = stacks.get_mut(m.to).unwrap();
            to.extend(items.iter().rev());
        }
    }
}

fn do_moves_p2(moves: Vec<Move>, stacks: &mut Vec<Stack>) {
    for m in moves.iter() {
        let mut items = Vec::new();
        {
            let from = stacks.get_mut(m.from).unwrap();
            items = from.split_off(from.len() - (m.count as usize));
        }
        {
            let to = stacks.get_mut(m.to).unwrap();
            to.extend(items);
        }
    }
}

fn solve1(data: &str) -> String {
    let (moves, mut stacks) = parse(data);
    do_moves_p1(moves, &mut stacks);
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn solve2(data: &str) -> String {
    let (moves, mut stacks) = parse(data);
    do_moves_p2(moves, &mut stacks);
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
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
        assert_eq!(solve1(data), "CMZ")
    }

    #[test]
    fn test_part1_regression() {
        let data = include_str!("../input.txt");
        assert_eq!(solve1(data), "MQTPGLLDN")
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../example.txt");
        assert_eq!(solve2(data), "MCD")
    }

    #[test]
    fn test_part2_regression() {
        let data = include_str!("../input.txt");
        assert_eq!(solve2(data), "LVZPSTTCZ")
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

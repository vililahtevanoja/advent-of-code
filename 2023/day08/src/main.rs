#![feature(test)]

extern crate test;

use std::collections::HashMap;

fn solve1(data: &str) -> usize {
    let (dirs_str, nodes_str) = data.split_once("\n\n").unwrap();
    let nodes = nodes_str
        .split("\n")
        .map(|l| {
            let (id, dir_ids_str) = l.split_once(" = ").unwrap();
            let dir_ids = dir_ids_str
                .chars()
                .filter(|c| c.is_alphabetic() || c.is_ascii_whitespace())
                .collect::<String>();
            let (left, right) = dir_ids.split_once(" ").unwrap();
            (id, (left.clone().to_owned(), right.clone().to_owned()))
        })
        .collect::<HashMap<_, _>>();
    let mut node = "AAA";
    dirs_str
        .as_bytes()
        .iter()
        .cycle()
        .enumerate()
        .find(|(_, direction)| {
            let (left, right) = nodes.get(node).unwrap();
            if **direction == b'L' {
                node = left;
            } else {
                node = right;
            }
            node == "ZZZ"
        })
        .map(|(count, _)| count + 1)
        .unwrap()
}

fn gcd(n1: usize, n2: usize) -> usize {
    let mut a = n1;
    let mut b = n2;
    let mut t = 0usize;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn solve2(data: &str) -> usize {
    let (dirs_str, nodes_str) = data.split_once("\n\n").unwrap();
    let nodes = nodes_str
        .split("\n")
        .map(|l| {
            let (id, dir_ids_str) = l.split_once(" = ").unwrap();
            let dir_ids = dir_ids_str
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
                .collect::<String>();
            let (left, right) = dir_ids.split_once(" ").unwrap();
            (id, (left.clone().to_owned(), right.clone().to_owned()))
        })
        .collect::<HashMap<_, _>>();
    let starts = nodes
        .iter()
        .map(|(id, _)| id)
        .filter(|node| node.ends_with("A"))
        .collect::<Vec<_>>();
    starts
        .iter()
        .map(|id| {
            let mut node = **id;
            dirs_str
                .as_bytes()
                .iter()
                .cycle()
                .enumerate()
                .find(|(_, direction)| {
                    let (left, right) = nodes
                        .get(node)
                        .unwrap_or_else(|| panic!("Tried to get node {}", node));
                    if **direction == b'L' {
                        node = left;
                    } else {
                        node = right;
                    }
                    node.ends_with("Z")
                })
                .map(|(count, _)| count + 1)
                .unwrap()
        })
        .reduce(|a, b| lcm(a, b))
        .unwrap()
}

fn main() {
    let data = include_str!("../input.txt");
    println!("Part 1: {}", solve1(data));
    println!("Part 2: {}", solve2(data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        let data = include_str!("../example.txt");
        assert_eq!(solve1(data), 2);
        let data2 = include_str!("../example2.txt");
        assert_eq!(solve1(data2), 6);
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../example3.txt");
        assert_eq!(solve2(data), 6)
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

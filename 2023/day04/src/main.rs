#![feature(test)]

use std::collections::{HashMap, HashSet};

extern crate test;

fn parse(data: &str) -> Vec<&str> {
    data.split("\n").collect()
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
}

impl Card {
    fn parse(s: &str) -> Card {
        let mut line_iterator = s.split(":");
        let id: usize = line_iterator
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut numbers_iterator = line_iterator.next().unwrap().split("|");
        let winning_numbers = numbers_iterator
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u8>().unwrap())
            .collect();
        let numbers = numbers_iterator
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u8>().unwrap())
            .collect();
        Card {
            id,
            winning_numbers,
            numbers,
        }
    }

    fn count_points(&self) -> usize {
        let winning_set: HashSet<u8> = self.winning_numbers.clone().into_iter().collect();
        let numbers_set: HashSet<u8> = self.numbers.clone().into_iter().collect();
        numbers_set
            .intersection(&winning_set)
            .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
    }

    fn won_cards(&self) -> Vec<usize> {
        let start = self.id + 1;
        let winning_set: HashSet<u8> = self.winning_numbers.clone().into_iter().collect();
        let numbers_set: HashSet<u8> = self.numbers.clone().into_iter().collect();
        let matching_cards_count = numbers_set.intersection(&winning_set).count();
        (start..start + matching_cards_count).collect()
    }
}

fn solve1(data: &str) -> usize {
    parse(data)
        .iter()
        .map(|l| Card::parse(l))
        .map(|c| c.count_points())
        .sum()
}

fn solve2(data: &str) -> usize {
    let cards = parse(data)
        .iter()
        .map(|l| Card::parse(l))
        .collect::<Vec<Card>>();
    let mut card_id_copies: HashMap<usize, usize> =
        HashMap::from_iter(cards.iter().map(|c| (c.id, 1)));
    cards.iter().for_each(|c| {
        c.won_cards().iter().for_each(|w| {
            let curr_card_copy_count = card_id_copies.get(&c.id).unwrap();
            let prev = card_id_copies.get(w).unwrap();
            card_id_copies.insert(*w, prev + curr_card_copy_count);
        })
    });
    card_id_copies.iter().map(|(_, won_times)| won_times).sum()
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
        assert_eq!(solve1(data), 13)
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../example.txt");
        assert_eq!(solve2(data), 30)
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

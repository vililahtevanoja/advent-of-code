#![feature(test)]

use std::{cmp::Ordering, collections::HashMap};

extern crate test;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum JokerHandling {
    NoJokers,
    Jokers,
}

#[derive(Debug, Clone, Copy, Eq)]
struct Hand {
    hand: [u8; 5],
    hand_type: HandType,
    bid: usize,
    jokers: JokerHandling,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl HandType {
    fn from_usize(n: usize) -> HandType {
        match n {
            1 => HandType::HighCard,
            2 => HandType::OnePair,
            3 => HandType::TwoPair,
            4 => HandType::ThreeOfAKind,
            5 => HandType::FullHouse,
            6 => HandType::FourOfAKind,
            7 => HandType::FiveOfAKind,
            _ => panic!("HandType not valid for {}", n),
        }
    }
}

impl HandType {
    fn detect(hand: [u8; 5]) -> HandType {
        let mut card_count_map: HashMap<u8, u8> = HashMap::new();
        hand.iter().for_each(|c| {
            let new_count = card_count_map.get(c).map(|c| c + 1).unwrap_or(1);
            card_count_map.insert(*c, new_count);
        });
        let counts = card_count_map
            .iter()
            .map(|(_, count)| *count)
            .collect::<Vec<_>>();
        let max_count = card_count_map
            .iter()
            .fold(0, |prev, (_, count2)| prev.max(*count2));
        return match max_count {
            5 => HandType::FiveOfAKind,
            4 => return HandType::FourOfAKind,
            3 => {
                if counts.iter().filter(|c| **c == 2).count() == 0 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            2 => {
                if counts.iter().filter(|c| **c == 2).count() == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => panic!("max count should not be {}", max_count),
        };
    }

    fn detect_with_jokers(hand: [u8; 5]) -> HandType {
        if hand.iter().all(|c| *c != 1) || hand.iter().all(|c| *c == 1) {
            // no jokers or all jokers => use default detection
            return Self::detect(hand);
        }
        let mut card_count_map: HashMap<u8, u8> = HashMap::new();
        hand.iter().for_each(|c| {
            let new_count = card_count_map.get(c).map(|c| c + 1).unwrap_or(1);
            card_count_map.insert(*c, new_count);
        });
        let joker_count = *card_count_map.get(&1).unwrap() as usize;
        let counts = card_count_map
            .iter()
            .map(|(_, count)| *count)
            .collect::<Vec<_>>();

        let max_non_joker_count = card_count_map
            .iter()
            .filter(|(c, _)| **c != 1)
            .fold(0, |prev, (_, count2)| prev.max(*count2));

        return match max_non_joker_count {
            5 => HandType::FiveOfAKind,
            4 => HandType::from_usize((HandType::FourOfAKind as usize) + joker_count),
            3 => {
                if joker_count == 2 {
                    HandType::FiveOfAKind
                } else {
                    HandType::FourOfAKind
                }
            }
            2 => {
                if counts.iter().filter(|c| **c == 2).count() == 2 {
                    // two pairs
                    if joker_count == 1 {
                        HandType::FullHouse
                    } else {
                        HandType::FourOfAKind
                    }
                } else {
                    // one pair
                    match joker_count {
                        1 => HandType::ThreeOfAKind,
                        2 => HandType::FourOfAKind,
                        3 => HandType::FiveOfAKind,
                        _ => panic!(
                            "Joker count {} should not be possible for one pair",
                            joker_count
                        ),
                    }
                }
            }
            1 => match joker_count {
                1 => HandType::OnePair,
                2 => HandType::ThreeOfAKind,
                3 => HandType::FourOfAKind,
                4 => HandType::FiveOfAKind,
                _ => panic!(
                    "Max joker count {} should not be possible for high card",
                    joker_count
                ),
            },
            _ => panic!("max non-joker count should not be {}", max_non_joker_count),
        };
    }
}

impl Hand {
    fn parse(s: &str, jokers: JokerHandling) -> Hand {
        let (hand_str, bid_str) = s.split_once(" ").unwrap();
        let hand: [u8; 5] = hand_str
            .chars()
            .map(|c| c.to_string())
            .map(|c| match c.as_str() {
                "T" => 10,
                "J" => match jokers {
                    JokerHandling::NoJokers => 11,
                    JokerHandling::Jokers => 1,
                },
                "Q" => 12,
                "K" => 13,
                "A" => 14,
                _ => c.parse::<u8>().unwrap(),
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        Hand {
            hand: hand,
            hand_type: match jokers {
                JokerHandling::NoJokers => HandType::detect(hand),
                JokerHandling::Jokers => HandType::detect_with_jokers(hand),
            },
            bid: bid_str.parse::<usize>().unwrap(),
            jokers,
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.jokers {
            JokerHandling::NoJokers => {
                match &self.hand_type.cmp(&other.hand_type) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => (),
                }
                self.hand.cmp(&other.hand)
            }
            JokerHandling::Jokers => {
                match &self.hand_type.cmp(&other.hand_type) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => (),
                }
                self.hand.cmp(&other.hand)
            }
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other) == Ordering::Equal
    }
}

fn parse(data: &str, jokers: JokerHandling) -> Vec<Hand> {
    data.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| Hand::parse(l, jokers))
        .collect::<Vec<_>>()
}

fn solve1(data: &str) -> usize {
    let mut hands = parse(data, JokerHandling::NoJokers);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1, hand))
        .map(|(rank, hand)| hand.bid * rank)
        .fold(0, |a, b| a + b)
}

fn solve2(data: &str) -> usize {
    let mut hands = parse(data, JokerHandling::Jokers);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1, hand))
        .map(|(rank, hand)| hand.bid * rank)
        .fold(0, |a, b| a + b)
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
        assert_eq!(solve1(data), 6440)
    }

    #[test]
    fn test_actual_part1() {
        let data = include_str!("../input.txt");
        assert_eq!(solve1(data), 245794640);
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../example.txt");
        assert_eq!(solve2(data), 5905)
    }

    #[test]
    fn test_actual_part2() {
        let data = include_str!("../input.txt");
        assert_eq!(solve2(data), 247899149);
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

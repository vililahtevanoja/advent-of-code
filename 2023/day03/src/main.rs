#![feature(test)]

use std::collections::HashSet;

extern crate test;

#[derive(PartialEq, Hash, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Item {
    coord: Coord,
    char: char,
}

#[derive(Hash, PartialEq, PartialOrd, Ord, Eq, Clone, Debug)]
struct Number {
    n: u16,
    coords: Vec<Coord>,
}

fn parse(data: &str) -> Vec<&str> {
    data.split("\n").collect()
}

const ADJACENT_COORD_OFFSETS: [(i16, i16); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn coord_is_adjacent_to_symbol(symbol_coords: &HashSet<Coord>, coord: &Coord) -> bool {
    ADJACENT_COORD_OFFSETS
        .iter()
        .filter(|(x, y)| !((coord.x == 0 && *x < 0) || (coord.y == 0 && *y < 0)))
        .map(|(x_offset, y_offset)| Coord {
            x: (coord.x as i16 + x_offset) as usize,
            y: (coord.y as i16 + y_offset) as usize,
        })
        .any(|c| symbol_coords.contains(&c))
}

fn items_to_nums(items: &Vec<Item>) -> Vec<Number> {
    let mut curr_num_str: String = String::from("");
    let mut curr_num_coords: Vec<Coord> = vec![];
    let mut nums: Vec<Number> = vec![];
    for item in items.iter() {
        if item.char.is_numeric() {
            curr_num_str.push(item.char);
            curr_num_coords.push(item.coord);
            continue;
        } else if curr_num_str.len() > 0 {
            nums.push(Number {
                n: curr_num_str.parse::<u16>().unwrap(),
                coords: curr_num_coords.clone(),
            });
            curr_num_str.clear();
            curr_num_coords.clear();
        }
    }
    nums
}

fn solve1(data: &str) -> u32 {
    let items: Vec<Item> = parse(data)
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| Item {
                coord: Coord { y, x },
                char: c,
            })
        })
        .collect();
    let symbol_coords = items
        .iter()
        .filter(|item| item.char != '.' && item.char.is_ascii_punctuation())
        .map(|item| item.coord)
        .collect::<HashSet<Coord>>();
    let nums = items_to_nums(&items);
    let nums_adjacent_to_symbol: Vec<&Number> = nums
        .iter()
        .filter(|num| {
            num.coords
                .iter()
                .any(|c| coord_is_adjacent_to_symbol(&symbol_coords, c))
        })
        .collect();
    nums_adjacent_to_symbol.iter().map(|num| num.n as u32).sum()
}

fn coord_adjacent_numbers(coord: &Coord, numbers: &Vec<Number>) -> Vec<Number> {
    let adjacent_numbers: HashSet<&Number> = HashSet::from_iter(
        ADJACENT_COORD_OFFSETS
            .iter()
            .filter(|(x, y)| !((coord.x == 0 && *x < 0) || (coord.y == 0 && *y < 0)))
            .map(|(x_offset, y_offset)| Coord {
                x: (coord.x as i16 + x_offset) as usize,
                y: (coord.y as i16 + y_offset) as usize,
            })
            .flat_map(|c| {
                numbers
                    .iter()
                    .filter(|n| n.coords.contains(&c))
                    .collect::<Vec<&Number>>()
            })
            .into_iter(),
    );
    adjacent_numbers
        .into_iter()
        .cloned()
        .collect::<Vec<Number>>()
}

fn solve2(data: &str) -> u32 {
    let items: Vec<Item> = parse(data)
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| Item {
                coord: Coord { y, x },
                char: c,
            })
        })
        .collect();
    let nums = items_to_nums(&items);
    let gear_candidates: Vec<&Item> = items
        .iter()
        .filter(|item| item.char == '*')
        .collect::<Vec<&Item>>();
    gear_candidates
        .iter()
        .map(|gc| coord_adjacent_numbers(&gc.coord, &nums))
        .filter(|ns| ns.len() == 2)
        .map(|ns| {
            ns.iter()
                .map(|n| n.n as u32)
                .reduce(|acc, n| acc * n)
                .unwrap()
        })
        .reduce(|acc, n| n + acc)
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
        assert_eq!(solve1(data), 4361)
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../example.txt");
        assert_eq!(solve2(data), 467835)
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

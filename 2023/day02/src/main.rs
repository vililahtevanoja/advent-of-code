#![feature(test)]

extern crate test;

fn parse(data: &str) -> Vec<&str> {
    data.split("\n").collect()
}

#[derive(Debug, Clone, Copy)]
struct CubeSet {
    red: u16,
    green: u16,
    blue: u16,
}

impl CubeSet {
    fn power(&self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }

    fn fits_within_constraint_set(&self, constraint_set: &CubeSet) -> bool {
        self.red <= constraint_set.red
            && self.green <= constraint_set.green
            && self.blue <= constraint_set.blue
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    fn is_possible_with_set(&self, set: &CubeSet) -> bool {
        self.sets
            .iter()
            .all(|gs| gs.fits_within_constraint_set(set))
    }
    fn min_possible_set(&self) -> CubeSet {
        self.sets.iter().fold(
            CubeSet {
                red: 0,
                green: 0,
                blue: 0,
            },
            |acc, cs| CubeSet {
                red: Ord::max(acc.red, cs.red),
                green: Ord::max(acc.green, cs.green),
                blue: Ord::max(acc.blue, cs.blue),
            },
        )
    }
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn parse_game(line: &str) -> Game {
    let id = line
        .split(":")
        .next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let sets = line
        .split(":")
        .last()
        .unwrap()
        .split(";")
        .map(|set| {
            set.split(",").fold(
                CubeSet {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |acc, s| {
                    let ss: Vec<&str> = s.trim().split(" ").collect();
                    let n = ss.get(0).unwrap().parse::<u16>().unwrap();
                    match ss.get(1).unwrap().as_ref() {
                        "red" => CubeSet { red: n, ..acc },
                        "green" => CubeSet { green: n, ..acc },
                        "blue" => CubeSet { blue: n, ..acc },
                        _ => panic!("Y here w/ no rgb? {}", s),
                    }
                },
            )
        })
        .collect();
    Game { id, sets }
}

fn solve1(data: &str) -> u32 {
    let test_set = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let lines = parse(data);
    lines
        .iter()
        .map(|l| parse_game(l))
        .filter(|g| g.is_possible_with_set(&test_set))
        .map(|g| g.id)
        .sum()
}

fn solve2(data: &str) -> u32 {
    let lines = parse(data);
    lines
        .iter()
        .map(|l| parse_game(l))
        .map(|g| g.min_possible_set())
        .map(|s| s.power())
        .sum()
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
        assert_eq!(solve1(data), 8)
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../example.txt");
        assert_eq!(solve2(data), 2286)
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

#![feature(test)]

use std::collections::HashMap;
extern crate test;

enum Operation {
    Addx(i64),
    Noop,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pixel {
    Dark,
    Lit,
}

impl Operation {
    fn from(s: &str) -> Operation {
        if s.trim() == "noop" {
            Operation::Noop
        } else {
            let (_, n) = s.split_once(" ").unwrap();
            Operation::Addx(n.parse::<i64>().unwrap())
        }
    }
    fn get_cycles(self) -> usize {
        match self {
            Operation::Noop => 1,
            Operation::Addx(_) => 2,
        }
    }
}

fn solve1(data: &str) -> i64 {
    let ops = data.lines().map(Operation::from);
    let mut cycle = 0usize;
    let mut counter = 1i64;
    let mut cycle_map: HashMap<usize, i64> = HashMap::with_capacity(240);
    for op in ops {
        match op {
            Operation::Noop => {
                cycle = cycle + 1;
                cycle_map.insert(cycle, counter);
            }
            Operation::Addx(n) => {
                for _ in 0..op.get_cycles() {
                    cycle = cycle + 1;
                    cycle_map.insert(cycle, counter);
                }
                counter = counter + n;
            }
        }
    }
    let checkpoints: [usize; 6] = [20, 60, 100, 140, 180, 220];
    checkpoints.iter().fold(0i64, |acc, cp| {
        acc + ((*cp as i64) * cycle_map.get(cp).unwrap())
    })
}

fn solve2(data: &str) -> String {
    let ops = data.lines().map(Operation::from);
    let mut cycle = 0usize;
    let mut counter = 1i64;
    let mut cycle_to_counter: Vec<(usize, i64)> = Vec::with_capacity(240);
    for op in ops {
        match op {
            Operation::Noop => {
                cycle += 1;
                cycle_to_counter.push((cycle, counter));
            }
            Operation::Addx(n) => {
                for _ in 0..2 {
                    cycle += 1;
                    cycle_to_counter.push((cycle, counter));
                }
                counter = counter + n;
            }
        }
    }
    let mut screen = [[Pixel::Dark; 40]; 6];
    for (cycle, counter) in cycle_to_counter.iter() {
        let column = (cycle - 1) % 40;
        let row = (cycle - 1) / 40;
        let column_i64 = column as i64;
        if column_i64 >= counter - 1 && column_i64 <= counter + 1 {
            screen[row][column] = Pixel::Lit;
        }
    }

    let s = screen
        .map(|r| r.map(|p| if p == Pixel::Dark { "." } else { "#" }).join(""))
        .join("\n");
    s
}

fn main() {
    let data = include_str!("../input.txt");
    println!("Part 1: {}", solve1(data));
    println!("Part 2: \n{}", solve2(data));
}

mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        let data = include_str!("../example.txt");
        assert_eq!(solve1(data), 13140)
    }

    #[test]
    fn test_part1_regression() {
        let data = include_str!("../input.txt");
        assert_eq!(solve1(data), 14540)
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../example.txt");
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(solve2(data), expected)
    }

    #[test]
    fn test_part2_regression() {
        let data = include_str!("../input.txt");
        let expected = "####.#..#.####.####.####.#..#..##..####.
#....#..#....#.#.......#.#..#.#..#....#.
###..####...#..###....#..####.#......#..
#....#..#..#...#.....#...#..#.#.....#...
#....#..#.#....#....#....#..#.#..#.#....
####.#..#.####.#....####.#..#..##..####.";
        assert_eq!(solve2(data), expected)
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

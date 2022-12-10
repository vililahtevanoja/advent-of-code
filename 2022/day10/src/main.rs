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
  Lit
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
  let mut cycle_to_counter: Vec<(usize, i64)> = Vec::new();
  for op in ops {
     match op {
      Operation::Noop => {
        cycle = cycle+1;
        cycle_to_counter.push((cycle.clone(), counter.clone()));
      },
      Operation::Addx(n) => {
        for _ in 0..Operation::Addx(n).get_cycles() {
          cycle = cycle+1;
          cycle_to_counter.push((cycle.clone(), counter.clone()));
        }
        counter = counter + n;
      }
     }
  }
  let cycle_map: HashMap<usize, i64> = cycle_to_counter.into_iter().collect();
  let mut sum = 0i64;
  let checkpoints: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
  for cp in checkpoints.iter() {
    let counter = cycle_map.get(cp).unwrap();
    sum = sum + (*cp as i64 * counter);
  }
  sum 
}

fn solve2(data: &str) -> String {
  let ops = data.lines().map(Operation::from);
  let mut cycle = 0usize;
  let mut counter = 1i64;
  let mut cycle_to_counter: Vec<(usize, i64)> = Vec::new();
  for op in ops {
     match op {
      Operation::Noop => {
        cycle = cycle+1;
        cycle_to_counter.push((cycle.clone(), counter.clone()));
      },
      Operation::Addx(n) => {
        for _ in 0..Operation::Addx(n).get_cycles() {
          cycle = cycle+1;
          cycle_to_counter.push((cycle.clone(), counter.clone()));
        }
        counter = counter + n;
      }
     }
  }
  assert_eq!(cycle_to_counter.len(), 240);
  let mut screen = [[Pixel::Dark;40]; 6];
  for (cycle, counter) in cycle_to_counter.iter() {
    let column = (cycle-1) % 40;
    let row = (cycle-1) / 40;
    if (counter-1..counter+2).contains(&(column as i64)) {
      screen[row][column] = Pixel::Lit;
    } 
  }
       
  let mut s = "".to_string();
  for row in screen {
    for p in row {
      s = s + match p {
        Pixel::Dark => ".",
        Pixel::Lit => "#",
      } 
    }
    s = s + "\n";
  }
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
#######.......#######.......#######.....
";
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
####.#..#.####.#....####.#..#..##..####.
";
      assert_eq!(solve2(data), expected)
    }
  
    #[bench]
    fn bench_part1(b: &mut Bencher) {
      let data = include_str!("../example.txt");
      b.iter(|| solve1(data));
    }
    
    #[bench]
    fn bench_part2(b: &mut Bencher) {
      let data = include_str!("../example.txt");
      b.iter(|| solve2(data));
    }
  }

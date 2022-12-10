#![feature(test)]

use std::collections::HashSet;
extern crate test;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    y: i64,
    x: i64,
}

enum Diagonality {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}


impl Coordinate {
  fn mv_once(&self, m: &Movement) -> Coordinate {
    match m {
      Movement::Right(_) => Coordinate { y: self.y, x: self.x+1 },
      Movement::Left(_) => Coordinate { y: self.y, x: self.x-1}, 
      Movement::Up(_) => Coordinate { y: self.y+1, x: self.x },
      Movement::Down(_) => Coordinate { y: self.y-1, x: self.x },
    }
  }

  fn is_in_same_row_or_column(&self, c: &Coordinate) -> bool {
    self.x == c.x || self.y == c.y
  }

  fn move_diagonally_towards(&self, c: &Coordinate) -> Coordinate {
    assert!(!self.is_in_same_row_or_column(c));
    match (self.y > c.y, self.x > self.y) {
      (true, true) => Coordinate {y: self.y+1, x: self.x+1},
      (true, false) => Coordinate { y: self.y+1, x: self.x-1 },
      (false, true) => Coordinate { y: self.y-1, x: self.x+1 },
      (false, false) => Coordinate { y: self.y-1, x: self.x-1 }
    } 
  }

  fn move_directly_towards(&self, c: &Coordinate) -> Coordinate {
    assert!(self.is_in_same_row_or_column(c));
    if self.y == c.y && self.x == c.x {
      return self.clone()
    } else if self.y != c.y {
      if self.y < c.y {
        self.mv_once(&Movement::Up(1))
      } else {
        self.mv_once(&Movement::Down(1))
      }
    } else {
      if self.x < c.x {
        self.mv_once(&Movement::Right(1))
      } else {
        self.mv_once(&Movement::Left(1))
      }
    }
  }
}

enum Movement {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Movement {
    fn from(data: &str) -> Movement {
        let (dir, count) = data.split_once(" ").unwrap();
        match dir {
            "R" => Movement::Right(count.parse::<usize>().unwrap()),
            "L" => Movement::Left(count.parse::<usize>().unwrap()),
            "U" => Movement::Up(count.parse::<usize>().unwrap()),
            "D" => Movement::Down(count.parse::<usize>().unwrap()),
            _ => panic!("Should not be here, received {}", dir)

        }
    }
    fn count(&self) -> usize {
        match self {
            Movement::Up(n) => *n,
            Movement::Down(n) => *n,
            Movement::Left(n) => *n,
            Movement::Right(n) => *n,
        }
    }
    
}

impl Coordinate {
    fn is_adjacent_to(&self, coord: &Coordinate) -> bool {
        self.y.abs_diff(coord.y) <= 1 && self.x.abs_diff(coord.x) <= 1
    }
}

fn solve1(data: &str) -> usize {
    let movements = data.lines().map(Movement::from);
    let mut tail_set: HashSet<Coordinate> = HashSet::new();
    let mut tail = Coordinate {y: 0, x: 0};
    let mut head = Coordinate {y: 0, x: 0};
    for movement in movements {
        for _ in 0..movement.count() {
            let head_last = head.clone();
            head = head.mv_once(&movement);
            if !tail.is_adjacent_to(&head) {
                tail = head_last;
            }
            tail_set.insert(tail.clone());
        }
    }
    tail_set.len()
}

fn solve2(data: &str) -> usize {
    let movements = data.lines().map(Movement::from);
    let mut tail_set: HashSet<Coordinate> = HashSet::new();
    let mut knots = vec![Coordinate {y: 0, x: 0}; 10];
    for movement in movements {
        for _ in 0..movement.count() {
            let mut previous: Option<Coordinate> = None;
            let mut previous_last = Coordinate {y:0,x:0};
            for knot in knots.iter_mut() {
                if let Some(ref prev_knot) = previous {
                   if !knot.is_adjacent_to(prev_knot) {
                        let old = knot.clone();
                       *knot = previous_last; 
                       previous_last = old;
                   }
                   previous = Some(knot.clone());
                } else { // head
                    let old = knot.clone();
                    *knot = knot.mv_once(&movement);
                    previous = Some(knot.clone());
                    previous_last = old;
                }
            }
            tail_set.insert(knots.last().unwrap().clone());
        }
    }
    tail_set.len()
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
        assert_eq!(solve1(data), 13)
    }

    #[test]
    fn test_part2() {
        let data1 = include_str!("../example.txt");
        assert_eq!(solve2(data1), 1);
        let data1 = include_str!("../example2.txt");
        assert_eq!(solve2(data1), 36)
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

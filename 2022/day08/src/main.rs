#![feature(test)]
extern crate test;

#[derive(Debug, Clone, Copy)]
struct Tree {
  height: i16,
  visible: bool,
}

type Forest = Vec<Vec<Tree>>;

trait ForestMethods {
  fn mark_visible_trees(&mut self);
  fn count_visible_trees(&self) -> usize;
  fn get_scenic_score(&self, x: usize, y: usize) -> usize;
  fn max_scenic_score(&self) -> usize;
  fn print(&self);
  fn get_xy(&self, x: usize, y: usize) -> &Tree;
}

impl ForestMethods for Forest {
  fn print(&self) {
    let rows = self.iter()
      .map(|l|l.iter().map(|t| if t.visible {"X"} else {"0"}).collect::<String>());
    rows.enumerate().for_each(|(i,r)| println!("{:0>2} {} {:0>2}", i, r, i))
  }
  fn mark_visible_trees(&mut self) {
    let rows = self.iter().len();
    let columns = self.iter().next().unwrap().len();
    for y in 0..rows {
      let mut last_max = -1; 
      for x in 0..columns {
        let mut tree = self.get_mut(y).unwrap().get_mut(x).unwrap(); 
        if tree.height > last_max {
          last_max = tree.height;
          tree.visible = true;
        }
      }
    }

    for y in 0..rows {
      let mut last_max = -1; 
      for x in (0..columns).rev() {
        let mut tree = self.get_mut(y).unwrap().get_mut(x).unwrap(); 
        if tree.height > last_max {
          last_max = tree.height;
          tree.visible = true;
        }
      }
    } 

    for x in 0..columns {
      let mut last_max = -1; 
      for y in 0..rows {
        let mut tree = self.get_mut(y).unwrap().get_mut(x).unwrap(); 
        if tree.height > last_max {
          last_max = tree.height;
          tree.visible = true;
        }
      }
    } 
    for x in 0..columns {
      let mut last_max = -1; 
      for y in (0..rows).rev() {
        let mut tree = self.get_mut(y).unwrap().get_mut(x).unwrap(); 
        if tree.height > last_max {
          last_max = tree.height;
          tree.visible = true;
        }
      }
    } 

  }

  fn count_visible_trees(&self) -> usize {
    self.iter().map(|r|r.iter().filter(|t|t.visible).count()).sum()
  } 

  fn get_scenic_score(&self, x: usize, y: usize) -> usize {
    let own_height = self.get_xy(x, y).height;
    let accumulator_f = |(count, stop), t: &Tree| {
      if stop {
        (count, stop)
      } else if t.height < own_height {
        (count+1, false)
      } else if t.height > own_height {
        (count+1, true)
      } else { // ==
        (count+1, true)
      }
    };
    let right = self.get(y).unwrap().iter()
      .skip(x+1)
      .fold((0usize, false), accumulator_f)
      .0;
    let left = self.get(y).unwrap().iter()
      .take(x)
      .rev()
      .fold((0usize, false), accumulator_f)
      .0;
    let down = self.iter().map(|r|r.get(x).unwrap())
      .skip(y+1)
      .fold((0usize, false), accumulator_f)
      .0;
    let up = self.iter().map(|r|r.get(x).unwrap())
      .take(y)
      .rev()
      .fold((0usize, false), accumulator_f)
      .0; 
    let score = right * left * down * up;
    score
  }

  fn max_scenic_score(&self) -> usize {
    let rows = self.iter().len();
    let columns = self.iter().next().unwrap().len();
    let mut current_max = 0;
    for y in 1..rows-1 {
      for x in 1..columns-1 {
        let score = self.get_scenic_score(x, y);
        if score > current_max {
          current_max = score;
        }
      }
    }

    current_max
  }

  fn get_xy(&self, x: usize, y: usize) -> &Tree{
    self.get(y).unwrap().get(x).unwrap()
  }

}


fn parse(data: &str) -> Forest {
  let zero_ascii = '0' as i16;
  data.lines()
    .map(|l| l.bytes().map(|c| Tree {height: c as i16 - zero_ascii, visible: false}).collect())
    .collect()
}

fn solve1(data: &str) -> usize {
  let mut parsed = parse(data);
  parsed.mark_visible_trees();
  parsed.count_visible_trees()
}

fn solve2(data: &str) -> usize {
  let parsed = parse(data);
  parsed.max_scenic_score()
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
      assert_eq!(solve1(data), 21)
  }

  #[test]
  fn test_part2() {
      let data = include_str!("../example.txt");
      assert_eq!(solve2(data), 8);
      let parsed = parse(data);
      assert_eq!(parsed.get_scenic_score(2, 3), 8, "Max score not in correct place");
      assert_eq!(parsed.get_scenic_score(2, 1), 4, "Check location score invalid => previous tests were right with invalid implementation");
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

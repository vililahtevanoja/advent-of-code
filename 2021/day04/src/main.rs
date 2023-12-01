#[derive(Debug, Clone, Copy)]
struct BingoNumber {
  number: u8,
  drawn: bool,
}

type Board = Vec<Vec<BingoNumber>>;

trait BoardMethods {
  fn marked(&self, n: u8) -> Board;
  fn wins(&self) -> bool;
}

fn transposed(m: &Vec<Vec<BingoNumber>>) -> Vec<Vec<BingoNumber>> {
  let mut m_trans: Vec<Vec<BingoNumber>> = vec![vec![BingoNumber::zero(); m.len()]; m[0].len()];
  for i in 0..m.len() {
    for j in 0..m[0].len() {
      m_trans[j][i] = m[i][j];
    }
  }
  m_trans
}

impl BoardMethods for Board {
  fn marked(&self, n: u8) -> Board {
    self.iter()
      .map(|l| l.iter().map(|bn| bn.tryMark(n)).collect())
      .collect()
  }

  fn wins(&self) -> bool {
    if self.iter().any(|l|l.iter().all(|bn| bn.drawn)) {
      return true
    } else {
      transposed(&self).iter()
      .any(|l|l.iter().all(|bn: &BingoNumber| bn.drawn))
    }
  }
}

#[derive(Debug, Clone)]
struct BingoGame {
  numbers: Vec<u8>,
  boards: Vec<Vec<Vec<BingoNumber>>>,
}

impl BingoGame {
  fn next(&self) -> BingoGame {
    let n = self.numbers[0];
    let mut new_boards: Vec<Board> = Vec::new();
    for board in self.boards.iter() {
      new_boards.push((&(board.marked(n)).to_owned()).to_vec());
    }
    
    // let new_boards = self.boards.iter().map(|b: &Board|&b.marked(n));
    // BingoGame { numbers: self.numbers[1..].to_vec(), boards: (new_boards.collect::<Vec<Board>>())}
    BingoGame { numbers: self.numbers[1..].to_vec(), boards: new_boards}
  }

  fn has_win(&self) -> bool {
    self.boards.iter().any(|b|b.wins())
  }
}

impl BingoNumber {
  fn from(s: &str) -> BingoNumber {
    let n = s.parse::<u8>().unwrap();
    BingoNumber { number: n, drawn: false }
  }
  fn marked(self) -> BingoNumber {
    BingoNumber { number: self.number, drawn: true }
  }
  fn tryMark(self, n: u8) -> BingoNumber {
    match self.number {
      n => BingoNumber{number: n, drawn: true},
      _ => self
    }
  }
  fn mark(&mut self) {
      self.drawn = true;
  }
  fn zero() -> BingoNumber {
    BingoNumber { number: 0, drawn: false}
  }
}

fn parse(data: &str) -> BingoGame {
  let lines: Vec<&str> = data.lines().collect();
  let numbers = lines[0].split(",").map(|s| s.parse::<u8>().unwrap()).collect(); 
  let boardLines: Vec<&&str> = lines.iter().skip(2).collect();
  let mut boards: Vec<Vec<Vec<BingoNumber>>> = Vec::new();
  let mut board: Vec<Vec<BingoNumber>> = Vec::new();
  for line in boardLines.iter() {
    if line.is_empty() {
      boards.push(board);
      board = Vec::new();
    }
    board.push(line.split_ascii_whitespace().map(BingoNumber::from).collect());
  }
  BingoGame { numbers: numbers, boards: boards }
}


fn solve1(data: &str) -> usize {
  let mut bingoGame = parse(data);
  println!("Game: {:?}", bingoGame);
  let mut round = 1;
  while !bingoGame.has_win() {
    bingoGame = bingoGame.next();
    println!("Game: {:?}", bingoGame);
    round += 1;
  }
  println!("Won in round {}", round);
  0
}

fn solve2(data: &str) -> usize {
  0
}


fn main() {
  let data = include_str!("../input.txt");
  println!("Part 1: {}", solve1(data));
  println!("Part 2: {}", solve2(data));
}

#[test]
fn test_part1() {
  let data = include_str!("../example.txt");
  assert_eq!(solve1(data), 4512)
}

#[test]
fn test_part2() {
  let data = include_str!("../example.txt");
  assert_eq!(solve2(data), 230);
}

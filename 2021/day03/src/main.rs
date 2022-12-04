use core::panic;
use std::collections::HashSet;


fn parse(data: &str) -> Vec<Vec<u8>> {
  data.lines()
    .map(|l| 
      l.chars().map(|c| c as u8 - '0' as u8).collect()
    ).collect() 
}

fn avg(l: Vec<u8>) -> f64 {
  let len = l.len() as f64;
  let sum: f64 = l.iter().fold(0 as u16, |n1, n2| n1+(*n2 as u16)) as f64;
  sum/len
}


fn most_common_binary(l: &Vec<u8>) -> u8 {
  match avg(l.to_vec()) {
    a if a < 0.5 => 0,
    a if a >= 0.5 => 1,
    a => panic!("Should not be here! {}", a)
  }
}

fn transposed(m: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let mut m_trans: Vec<Vec<u8>> = vec![vec![0; m.len()]; m[0].len()];
  for i in 0..m.len() {
    for j in 0..m[0].len() {
      m_trans[j][i] = m[i][j];
    }
  }
  m_trans
}

fn binary_to_int(v: Vec<u8>) -> u64 {
  let mut value = 0_u64;
  for (pos, i) in (0..v.len()).rev().enumerate() {
    let factor = 2_u64.pow(pos.try_into().unwrap());
    if v[i] == 1 {
      value += factor;
    }
  }
  value
}

fn solve1(data: &str) -> u64 {
  let parsed = parse(data);
  let transposed = transposed(&parsed);
  let most_commons: Vec<u8> = transposed.iter().map(most_common_binary).collect();
  let least_commons = most_commons.iter().map(|n| if *n == 1 as u8 {0} else {1}).collect();
  let gamma = binary_to_int(most_commons);
  let epsilon = binary_to_int(least_commons);
  gamma * epsilon
}

fn solve2(data: &str) -> u64 {
  let mut working_oxygen = parse(data);
  let mut working_co2 = parse(data);
  let mut i: usize = 0;
  while working_oxygen.len() > 1 {
    let trans = transposed(&working_oxygen); // transpose to easily read certain index in all vectors
    let most_common = most_common_binary(&trans[i]); // find out most common value in current index
    let indices = trans[i].iter().enumerate()
      .filter(|(n, item)| **item == most_common)
      .map(|(i, _)| i)
      .collect::<HashSet<usize>>();
    working_oxygen = working_oxygen.iter().enumerate()
        .filter(|(i, _)| indices.contains(i) )
        .map(|(_, item)| item.clone())
      .collect();
    i += 1;
  }
  let oxygen = binary_to_int(working_oxygen.first().unwrap().to_vec());
  i = 0;
  while working_co2.len() > 1 {
    let trans = transposed(&working_co2); // transpose to easily read certain index in all vectors
    let most_common = most_common_binary(&trans[i]); // find out most common value in current index
    let indices = trans[i].iter().enumerate()
      .filter(|(n, item)| **item != most_common)
      .map(|(i, _)| i)
      .collect::<HashSet<usize>>();
    working_co2 = working_co2.iter().enumerate()
        .filter(|(i, _)| indices.contains(i) )
        .map(|(_, item)| item.clone())
      .collect();
    i += 1;
  }

  let co2 = binary_to_int(working_co2.first().unwrap().to_vec());
  oxygen * co2
}

fn main() {
  let data = include_str!("../input.txt");
  println!("Part 1: {}", solve1(data));
  println!("Part 2: {}", solve2(data));
}

#[test]
fn test_part1() {
  let data = include_str!("../example.txt");
  assert_eq!(solve1(data), 198)
}

#[test]
fn test_part2() {
  let data = include_str!("../example.txt");
  assert_eq!(solve2(data), 230);
}

#[test]
fn test_binary_to_int() {
  let data: Vec<u8> = vec![0, 1, 0, 0, 1];
  assert_eq!(binary_to_int(data), 9)
}

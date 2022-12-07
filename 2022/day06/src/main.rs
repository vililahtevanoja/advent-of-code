use std::collections::HashSet;

const PACKET_START_MARKER_SIZE: usize = 4;

fn first_unique_window_ends_at(data: &str, window_size: usize) -> usize {
  let data_slice = &data.chars().collect::<Vec<char>>()[..];
  let data_windows = data_slice.windows(window_size);
  let mut hs : HashSet<&char>= HashSet::with_capacity(window_size);
  data_windows.enumerate().find_map(|(i,chars)| {
    hs.extend(chars.iter());
    let hs_len = hs.len();
    hs.clear();
    match hs_len {
      _ if hs_len == window_size => Some(i+window_size),
      _ => None,
    }
  }).unwrap()
}

fn solve1(data: &str) -> usize {
  first_unique_window_ends_at(data, PACKET_START_MARKER_SIZE)
}

const MESSAGE_START_MARKER_SIZE: usize = 14;

fn solve2(data: &str) -> usize {
  first_unique_window_ends_at(data, MESSAGE_START_MARKER_SIZE)
}


fn main() {
  let data = include_str!("../input.txt");
  println!("Part 1: {}", solve1(data));
  println!("Part 2: {}", solve2(data));
}

#[test]
fn test_part1() {
  let data = include_str!("../example.txt");
  assert_eq!(solve1(data), 7);
}

#[test]
fn test_part2() {
  let data = include_str!("../example.txt");
  assert_eq!(solve2(data), 19);
}

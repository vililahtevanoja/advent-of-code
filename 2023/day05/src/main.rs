#![feature(test)]

use rayon::prelude::*;
use std::ops::Range;

extern crate test;

#[derive(Debug, Clone)]
struct MapEntry {
    dest_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl MapEntry {
    fn parse(s: &str) -> MapEntry {
        let mut it = s.split_ascii_whitespace();
        let dest_range_start = it.next().unwrap().parse::<usize>().unwrap();
        let source_range_start = it.next().unwrap().parse::<usize>().unwrap();
        let range_length = it.next().unwrap().parse::<usize>().unwrap();
        MapEntry {
            dest_range_start,
            source_range_start,
            range_length,
        }
    }
}

fn parse(data: &str) -> (Vec<usize>, Vec<Vec<MapEntry>>) {
    let segments = data
        .split("\n\n")
        .map(|s| s.split("\n").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let seeds = segments
        .first()
        .unwrap()
        .first()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let maps = segments
        .iter()
        .map(|s| {
            s.iter()
                .skip(1)
                .filter(|s| !s.is_empty())
                .map(|s| MapEntry::parse(s))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (seeds, maps)
}

fn solve1(data: &str) -> usize {
    let (seeds, maps) = parse(data);
    seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |prev, map| {
                map.iter()
                    .find(|entry| {
                        prev >= entry.source_range_start
                            && prev < entry.source_range_start + entry.range_length
                    })
                    .map(|entry| {
                        let range_idx = prev - entry.source_range_start;
                        entry.dest_range_start + range_idx
                    })
                    .unwrap_or(prev)
            })
        })
        .min()
        .unwrap()
}

fn parse2(data: &str) -> (Vec<Range<usize>>, Vec<Vec<MapEntry>>) {
    let (seeds, maps) = parse(data);
    let seed_ranges: Vec<Range<usize>> = seeds
        .chunks_exact(2)
        .map(|c| {
            let start = *c.get(0).unwrap();
            let length = *c.get(1).unwrap();
            Range {
                start,
                end: start + length,
            }
        })
        .collect();
    (seed_ranges, maps)
}

fn solve2(data: &str) -> usize {
    let (seed_ranges, maps) = parse2(data);
    seed_ranges
        .iter()
        .map(|sr| {
            sr.clone()
                .into_par_iter()
                .map(|seed| {
                    maps.iter().fold(seed, |prev, map| {
                        map.iter()
                            .find(|entry| {
                                prev >= entry.source_range_start
                                    && prev < entry.source_range_start + entry.range_length
                            })
                            .map(|entry| prev - entry.source_range_start + entry.dest_range_start)
                            .unwrap_or(prev)
                    })
                })
                .min()
                .unwrap()
        })
        .min()
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
        assert_eq!(solve1(data), 35)
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../example.txt");
        assert_eq!(solve2(data), 46)
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let data = include_str!("../input.txt");
        b.iter(|| solve1(data));
    }

    // #[bench]
    // fn bench_part2(b: &mut Bencher) {
    //     let data = include_str!("../input.txt");
    //     b.iter(|| solve2(data));
    // }
}

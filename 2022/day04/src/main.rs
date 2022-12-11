#![feature(test)]
extern crate test;

#[derive(Debug, Clone, Copy)]
struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn from(s: &str) -> Section {
        let (start, end) = s.split_once("-").unwrap();
        Section {
            start: start.parse::<u32>().unwrap(),
            end: end.parse::<u32>().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SectionPair {
    first: Section,
    second: Section,
}

impl SectionPair {
    fn from(s: &str) -> SectionPair {
        let (first, second) = s.split_once(",").unwrap();
        SectionPair {
            first: Section::from(first),
            second: Section::from(second),
        }
    }

    fn one_contains_other(self) -> bool {
        (self.first.start <= self.second.start && self.first.end >= self.second.end)
            || (self.second.start <= self.first.start && self.second.end >= self.first.end)
    }

    fn overlaps(self) -> bool {
        !(self.first.end < self.second.start || self.first.start > self.second.end)
    }
}

fn solve1(data: &str) -> usize {
    data.lines()
        .map(SectionPair::from)
        .filter(|sp| sp.one_contains_other())
        .count()
}

fn solve2(data: &str) -> usize {
    data.lines()
        .map(SectionPair::from)
        .filter(|sp| sp.overlaps())
        .count()
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
        assert_eq!(solve1(data), 2)
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../example.txt");
        assert_eq!(solve2(data), 4)
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

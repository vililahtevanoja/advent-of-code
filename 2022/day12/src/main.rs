#![feature(test)]
extern crate test;

fn solve1(data: &str) -> usize {
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

mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        let data = include_str!("../example.txt");
        assert_eq!(solve1(data), 31)
    }

    #[test]
    fn test_part1_regression() {
        let data = include_str!("../input.txt");
        assert_eq!(solve1(data), 6236)
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../example.txt");
        assert_eq!(solve2(data), 1);
    }

    #[test]
    fn test_part2_regression() {
        let data = include_str!("../input.txt");
        assert_eq!(solve2(data), 2449)
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

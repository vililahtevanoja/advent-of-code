#![feature(test)]

extern crate test;

trait AllZerosCheck {
    fn all_zeros(&self) -> bool;
}

impl AllZerosCheck for Vec<i64> {
    fn all_zeros(&self) -> bool {
        self.iter().all(|n| *n == 0)
    }
}

fn parse_histories(data: &str) -> Vec<Vec<i64>> {
    data.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn solve1(data: &str) -> usize {
    let histories = parse_histories(data);
    histories
        .iter()
        .map(|hs| {
            let mut diff_seqs: Vec<Vec<i64>> = vec![hs.clone()];
            diff_seqs.push(hs.windows(2).map(|w| w[1] - w[0]).collect());
            loop {
                let last = diff_seqs.last().unwrap();
                if last.all_zeros() {
                    break;
                }
                diff_seqs.push(last.windows(2).map(|w| w[1] - w[0]).collect());
            }
            diff_seqs
        })
        .map(|diffs| {
            diffs
                .iter()
                .map(|ds| *ds.last().unwrap())
                .reduce(|prev, curr| prev + curr)
                .unwrap()
        })
        .reduce(|a, b| a + b)
        .unwrap() as usize
}

fn solve2(data: &str) -> usize {
    let reversed_histories = parse_histories(data)
        .iter()
        .map(|h| {
            let mut to_reverse = h.clone();
            to_reverse.reverse();
            to_reverse
        })
        .collect::<Vec<Vec<i64>>>();
    reversed_histories
        .iter()
        .map(|head| {
            let mut diff_seqs: Vec<Vec<i64>> = vec![head.clone()];
            diff_seqs.push(head.windows(2).map(|w| w[1] - w[0]).collect());
            loop {
                let last = diff_seqs.last().unwrap();
                if last.all_zeros() {
                    break;
                }
                diff_seqs.push(last.windows(2).map(|w| w[1] - w[0]).collect());
            }
            diff_seqs
        })
        .map(|diffs| {
            diffs
                .iter()
                .map(|ds| *ds.last().unwrap())
                .reduce(|prev, curr| prev + curr)
                .unwrap()
        })
        .reduce(|a, b| a + b)
        .unwrap() as usize
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
        assert_eq!(solve1(data), 114);
        let actual_data = include_str!("../input.txt");
        assert_eq!(solve1(actual_data), 1702218515);
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../example.txt");
        assert_eq!(solve2(data), 2);
        let actual_data = include_str!("../input.txt");
        assert_eq!(solve2(actual_data), 925);
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

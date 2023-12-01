#![feature(test)]

extern crate test;

fn parse(data: &str) -> Vec<&str> {
    data.split("\n").collect()
}

fn solve1(data: &str) -> u32 {
    let lines = parse(data);
    lines
        .iter()
        .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<String>())
        .map(|l| format!("{}{}", l.chars().next().unwrap(), l.chars().last().unwrap()))
        .map(|l| l.parse::<u32>().unwrap())
        .sum()
}

fn solve2(data: &str) -> u32 {
    let digit_strings_to_nums: Vec<(&str, &str)> = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let digit_strings_rev_to_nums: Vec<(String, String)> = digit_strings_to_nums
        .iter()
        .map(|(s, n)| (s.chars().rev().collect::<String>(), String::from(n.clone())))
        .collect();
    let digit_strings: Vec<String> = digit_strings_to_nums
        .iter()
        .map(|(s, _)| String::from(s.clone()))
        .collect();
    let digit_strings_rev: Vec<String> = digit_strings_rev_to_nums
        .iter()
        .map(|(s, _)| String::from(s))
        .collect();
    let lines = parse(data);

    let first_and_last_digits: Vec<String> = lines
        .iter()
        .map(|l| {
            let first_digit_start = (0..l.len())
                .map(|n| l.chars().skip(n).collect::<String>())
                .find(|c| {
                    c.chars().next().unwrap().is_numeric()
                        || digit_strings
                            .iter()
                            .find(|s| c.starts_with(s.clone()))
                            .is_some()
                })
                .unwrap();
            let first_digit = if first_digit_start.chars().next().unwrap().is_numeric() {
                first_digit_start.chars().take(1).collect::<String>()
            } else {
                digit_strings_to_nums
                    .iter()
                    .find(|(s, _)| first_digit_start.starts_with(s))
                    .map(|(_, n)| n)
                    .unwrap()
                    .to_string()
            };

            let last_digit_start = (0..l.len())
                .map(|n| l.chars().rev().skip(n).collect::<String>())
                .find(|c| {
                    c.chars().next().unwrap().is_numeric()
                        || digit_strings_rev
                            .iter()
                            .find(|s| c.starts_with(s.clone()))
                            .is_some()
                })
                .unwrap();
            let last_digit = if last_digit_start.chars().next().unwrap().is_numeric() {
                last_digit_start.chars().take(1).collect::<String>()
            } else {
                digit_strings_rev_to_nums
                    .iter()
                    .find(|(s, _)| last_digit_start.starts_with(s))
                    .map(|(_, n)| n)
                    .unwrap()
                    .to_string()
            };
            format!("{}{}", first_digit, last_digit)
        })
        .collect();

    first_and_last_digits
        .iter()
        .map(|n| n.parse::<u32>().unwrap())
        .sum()
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
        assert_eq!(solve1(data), 142)
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../example2.txt");
        assert_eq!(solve2(data), 281)
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

#![feature(test)]

extern crate test;

#[derive(Debug, Clone, Copy)]
struct Race {
    time: usize,
    best_distance: usize,
}

impl Race {
    fn get_button_hold_times_to_win(&self) -> Vec<usize> {
        (1..self.time)
            .map(|time_pressed| {
                let speed = time_pressed;
                let distance = (self.time - time_pressed) * speed;
                (time_pressed, distance)
            })
            .skip_while(|(_, distance)| distance <= &self.best_distance)
            .take_while(|(_, distance)| distance > &self.best_distance)
            .map(|(time_pressed, _)| time_pressed)
            .collect::<Vec<_>>()
    }
}

fn parse(data: &str) -> Vec<Race> {
    let lines = data.split("\n").collect::<Vec<_>>();
    let times = lines
        .first()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let distance = lines
        .last()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    times
        .iter()
        .zip(distance)
        .map(|(t, d)| Race {
            time: *t,
            best_distance: d,
        })
        .collect::<Vec<_>>()
}

fn solve1(data: &str) -> usize {
    let races = parse(data);
    races
        .iter()
        .map(|r| r.get_button_hold_times_to_win().iter().count())
        .reduce(|acc, curr| acc * curr)
        .unwrap()
}

fn parse2(data: &str) -> Race {
    let lines = data.split("\n").collect::<Vec<_>>();
    let time_line = lines.first().unwrap();
    let distance_line = lines.last().unwrap();
    let time = time_line
        .split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distance = distance_line
        .split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    Race {
        time,
        best_distance: distance,
    }
}

fn solve2(data: &str) -> usize {
    let race = parse2(data);
    race.get_button_hold_times_to_win().len()
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
        assert_eq!(solve1(data), 288)
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../example.txt");
        assert_eq!(solve2(data), 71503)
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

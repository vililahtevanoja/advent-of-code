#![feature(test)]

use std::{collections::HashMap, rc::Rc};
extern crate test;

#[derive(Debug)]
enum OutputItem {
    Cd(String),
    Ls,
    LsFileOutput(usize, String),
    LsDirOutput(String),
}

struct FS {
    parent: Option<Rc<FS>,
    children: Rc<HashMap<String, FS>>,
    size: usize,
}
´´
impl OutputItem {
    fn from(data: &str) -> OutputItem {
        match (data.starts_with("$"), data.ends_with("ls")) {
            (true, false) => OutputItem::Cd(data.split_once("cd").unwrap().1.trim().to_string()),
            (true, true) => OutputItem::Ls,
            (false, _) => {
                println!("Data: {}", data);
                let (size, filename) = data.split_once(" ").unwrap();
                match size {
                    "dir" => OutputItem::LsDirOutput(size.to_string()),
                    _ => OutputItem::LsFileOutput(
                        size.parse::<usize>().unwrap(),
                        filename.to_string(),
                    ),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum FS {
    Dir(String, Option<usize>, Vec<FS>),
    File(String, usize),
}

impl FS {
    fn print(&self) {
        match self {
            FS::Dir(name, size, contents) => {
                println!("{} {:?}", name, size);
                contents.iter().for_each(|fs| fs.print());
            }
            FS::File(name, size) => print!("{} {}", name, size),
        }
    }
    fn build(data: &str) {}
}

fn parse(data: &str) -> Vec<OutputItem> {
    data.lines().map(OutputItem::from).collect()
}

fn build_fs(items: Vec<OutputItem>) -> FS {
    let mut fs = FS::Dir("/".to_string(), None, Vec::new());
    let mut curr = fs;
    let mut previous = FS::Dir("".to_string(), None, vec![]);
    for item in items[1..].iter() {
        match item {
            Cd(dirname) => matc,
        }
    }
}

fn solve1(data: &str) -> usize {
    let items = parse(data);
    println!("{:?}", data);
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
        assert_eq!(solve1(data), 95437)
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../example.txt");
        assert_eq!(solve2(data), 230)
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

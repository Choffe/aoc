use std::cmp::max;
use std::collections::VecDeque;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "data/5/input";
const TEST_INPUT: &str = "data/5/test";

#[derive(Debug, Copy, Clone)]
struct Range {
    start: usize,
    stop: usize,
}

impl Range {
    fn inside(&self, i: usize) -> bool {
        i <= self.stop && i >= self.start
    }
    fn extend(&mut self, other: &Range) {
        self.stop = max(self.stop, other.stop);
    }
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("-");
        Ok(Range {
            start: split.next().unwrap().parse()?,
            stop: split.next().unwrap().parse()?,
        })
    }
}

fn part1(input: &str) -> usize {
    let s = fs::read_to_string(input).unwrap();
    let input: Vec<&str> = s.split("\n\n").collect();
    let valid_ranges: Vec<Range> = input[0].lines().map(|x| x.parse().unwrap()).collect();
    input[1]
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .filter(|x| valid_ranges.iter().any(|r| r.inside(*x)))
        .count()
}

fn part2(input: &str) -> usize {
    let s = fs::read_to_string(input).unwrap();
    let input: Vec<&str> = s.split("\n\n").collect();
    let mut valid_ranges: Vec<Range> = input[0].lines().map(|x| x.parse().unwrap()).collect();

    valid_ranges.sort_by(|x, y| x.start.cmp(&y.start));
    let mut total: Vec<Range> = Vec::new();

    total.push(valid_ranges[0]);

    for i in 1..valid_ranges.len() {
        let len = total.len();
        let mut last = &mut total[len - 1];
        let current = &valid_ranges[i];
        if last.stop + 1 >= current.start {
            last.stop = max(current.stop, last.stop);
        } else {
            total.push(valid_ranges[i]);
        }
    }

    total.iter().fold(0, |acc, x| acc + (x.stop - x.start) + 1)
}
// 329744896960197 too low

pub fn run() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 3);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 14);
}

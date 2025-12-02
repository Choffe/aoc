use std::{fs, str::FromStr};

const INPUT: &str = "data/2/input";
const TEST_INPUT: &str = "data/2/test";

#[derive(Debug)]
struct Range {
    start: i128,
    stop: i128,
    acc_invalid_id: i128,
}

impl Range {
    fn is_invalid_part1(i: i128) -> bool {
        let number_string = i.to_string();
        if !number_string.len().is_multiple_of(2) {
            false
        } else {
            let (first, second) = number_string.split_at(number_string.len() / 2);
            first.eq(second)
        }
    }

    fn is_invalid_part2(i: i128) -> bool {
        let number_string = i.to_string();
        for i in 1..number_string.len() {
            if !number_string.len().is_multiple_of(i) {
                continue;
            }
            let number_of_repetitions = number_string.len() / i;
            let (pattern, _) = number_string.split_at(i);
            let proposal = pattern.repeat(number_of_repetitions);
            if proposal.eq(&number_string) {
                return true;
            }
        }
        false
    }

    fn calculate_part1(&mut self) {
        for i in self.start..=self.stop {
            self.acc_invalid_id += if Range::is_invalid_part1(i) { i } else { 0 };
        }
    }

    fn calculate_part2(&mut self) {
        for i in self.start..=self.stop {
            self.acc_invalid_id += if Range::is_invalid_part2(i) { i } else { 0 };
        }
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range: Vec<i128> = s.trim().split("-").map(|x| x.parse().unwrap()).collect();
        assert!(range.len() == 2);
        Ok(Range {
            start: range[0],
            stop: range[1],
            acc_invalid_id: 0,
        })
    }
}

fn part1(input: &str) -> i128 {
    let s = fs::read_to_string(input).unwrap();

    s.trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .fold(0, |acc: i128, mut range: Range| {
            range.calculate_part1();
            acc + range.acc_invalid_id
        })
}

fn part2(input: &str) -> i128 {
    let s = fs::read_to_string(input).unwrap();

    s.trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .fold(0, |acc: i128, mut range: Range| {
            range.calculate_part2();
            acc + range.acc_invalid_id
        })
}

pub fn run() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 1227775554);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 4174379265);
}

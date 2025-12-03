use core::panic;
use std::{fs, str::FromStr};

const INPUT: &str = "data/3/input";
const TEST_INPUT: &str = "data/3/test";

#[derive(Debug)]
struct BatteryBank {
    batteries: Vec<u32>,
    highest_pair: u32,
    highest_twelve: u128,
}

impl BatteryBank {
    fn new(batteries: Vec<u32>) -> BatteryBank {
        let mut b = BatteryBank {
            batteries,
            highest_pair: 0,
            highest_twelve: 0,
        };
        b.compute_highest_pair();
        b.compute_highest_twelve();
        b
    }

    fn compute_highest_pair(&mut self) {
        let (_, rest) = self.batteries.split_last().unwrap();
        let first = *rest.iter().max().unwrap();
        let mut it = self.batteries.iter().skip_while(|x| **x != first);
        it.next();
        let second = *it.max().unwrap();
        self.highest_pair = first * 10 + second;
    }

    fn compute_highest_twelve(&mut self) {
        assert!(self.batteries.len() >= 12);
        let mut rest = self.batteries.clone();
        for i in (0..12).rev() {
            let max = rest[..rest.len() - i].iter().max().unwrap();
            let mut it = rest.iter().skip_while(|x| **x != *max);
            it.next();

            self.highest_twelve += *max as u128 * 10_u128.pow(i as u32);

            rest = it.copied().collect();
        }
    }
}

impl FromStr for BatteryBank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BatteryBank::new(
            s.chars().map(|c| c.to_digit(10).unwrap()).collect(),
        ))
    }
}

fn part1(input: &str) -> u32 {
    let s = fs::read_to_string(input).unwrap();

    s.trim()
        .lines()
        .map(|x| x.parse().unwrap())
        .fold(0, |acc, x: BatteryBank| acc + x.highest_pair)
}

fn part2(input: &str) -> u128 {
    let s = fs::read_to_string(input).unwrap();

    s.trim()
        .lines()
        .map(|x| x.parse().unwrap())
        .fold(0, |acc, x: BatteryBank| acc + x.highest_twelve)
}

pub fn run() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 357);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 3121910778619);
}

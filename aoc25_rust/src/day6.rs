use core::num;
use std::cmp::max;
use std::collections::VecDeque;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "data/6/input";
const TEST_INPUT: &str = "data/6/test";

fn part1(input: &str) -> i128 {
    let s = fs::read_to_string(input).unwrap();
    let input: Vec<Vec<&str>> = s
        .trim()
        .lines()
        .map(|x| x.split_whitespace().collect())
        .collect();
    let last_line: &Vec<&str> = &input[input.len() - 1];
    let number_columns = input[0].len();

    let mut columns = vec![0; number_columns];
    for col in 0..columns.len() {
        if last_line[col] == "*" {
            columns[col] = 1;
        }
    }

    for row in 0..(input.len() - 1) {
        for col in 0..input[row].len() {
            match last_line[col] {
                "+" => columns[col] += input[row][col].parse::<i128>().unwrap(),
                "*" => columns[col] *= input[row][col].parse::<i128>().unwrap(),
                c => panic!("Error on last line: {} -{}-", last_line.join(" "), c),
            };
        }
    }

    columns.iter().fold(0, |acc, x| acc + *x)
}

fn part2(input: &str) -> usize {
    0
}

pub fn run() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 4277556);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 3263827);
}

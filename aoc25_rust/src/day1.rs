use std::fs;
const INPUT: &str = "data/1/input";
const TEST_INPUT: &str = "data/1/test";

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<i32> = s.trim().split('\n').map(|x| x.parse().unwrap()).collect();
    lines[0]
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<i32> = s.trim().split('\n').map(|x| x.parse().unwrap()).collect();
    lines[0]
}

pub fn run() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 1);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 1);
}

use std::fs;

pub fn run() {
    let first = part1("data/1");
    println!("{}", first);
    let second = part2("data/1");
    println!("{}", second);
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = s.split('\n').collect();
    0
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = s.split('\n').collect();
    0
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/1_easy"), 1);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/1_easy"), 1);
}

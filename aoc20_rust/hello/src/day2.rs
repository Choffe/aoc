use regex::Regex;
use std::fs;

pub fn run() {
    let first = part1("data/2");
    println!("{}", first);
    let second = part2("data/2");
    println!("{}", second);
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = s.split('\n').collect();
    // 2-9 c: ccccccccc
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): (\w+)").unwrap();

    let mut valid_passwords = 0;
    for line in lines {
        for cap in re.captures_iter(line) {
            let from: usize = cap[1].parse().expect("From is not a number");
            let to: usize = cap[2].parse().expect("To is not a number");
            let letter = &cap[3];
            let password = &cap[4];
            let count = password
                .chars()
                .filter(|x| x == &letter.chars().next().unwrap())
                .count();
            if count >= from && count <= to {
                valid_passwords += 1;
            }
        }
    }
    valid_passwords
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = s.split('\n').collect();
    // 2-9 c: ccccccccc
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): (\w+)").unwrap();

    let mut valid_passwords = 0;
    for line in lines {
        for cap in re.captures_iter(line) {
            let first: usize = cap[1].parse().expect("First is not a number");
            let second: usize = cap[2].parse().expect("Second is not a number");
            let letter = &cap[3];
            let password = &cap[4];
            let valid = password
                .char_indices()
                .filter(|(i, v)| {
                    (i + 1 == first || i + 1 == second) && v == &letter.chars().next().unwrap()
                })
                .count();
            if valid == 1 {
                valid_passwords += 1
            }
        }
    }
    valid_passwords
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/2_easy"), 2);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/2_easy"), 1);
}

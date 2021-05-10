use std::fs;
use std::collections::HashMap;

pub fn run() {
    let first = part1("data/6");
    println!("{}", first);
    let second = part2("data/6");
    println!("{}", second);
}

fn part1(input: &str) -> usize {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = s.split("\n\n").collect();
    let mut sum = 0;
    for line in lines {
        let mut map: HashMap<char, u8> = HashMap::new();
        for c in line.chars() {
            if c != '\n' {
                map.insert(c, 0);
            }
        }
        sum += map.iter().count();
    }
    sum
}

//guessed 3484 -> too low
fn part2(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = s.split("\n\n").collect();
    let mut sum: i64 = 0;
    for group in lines {
        let mut map: HashMap<char, usize> = HashMap::new();
        for c in group.chars() {
            if c == '\n' {
                continue;
            }
            if map.contains_key(&c) {
                *map.get_mut(&c).unwrap() += 1;
            } else {
                map.insert(c, 1);
            }
        }
        let members = group.split("\n").collect::<Vec<&str>>().len();
        sum += map.iter().filter(|(_, &v)| v == members).count() as i64;
    }
    sum
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/6_easy"), 11);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/6_easy"), 6);
}


use std::fs;

pub fn run() {
    let first = part1("data/1");
    println!("{}", first);
    let second = part2("data/1");
    println!("{}", second);
}

fn increasing_cnt(vector: Vec<i32>) -> i32 {
    vector.iter().zip(vector[1..].iter()).fold(
        0,
        |acc, (first, second)| {
            if first < second {
                acc + 1
            } else {
                acc
            }
        },
    )
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<i32> = s.trim().split('\n').map(|x| x.parse().unwrap()).collect();
    increasing_cnt(lines)
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<i32> = s.trim().split('\n').map(|x| x.parse().unwrap()).collect();
    let mut windowed = vec![];
    for i in 0..lines.len() - 2 {
        windowed.push(lines[i] + lines[i + 1] + lines[i + 2])
    }
    increasing_cnt(windowed)
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/1_easy"), 7);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/1_easy"), 5);
}

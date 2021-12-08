use std::collections::HashMap;
use std::fs;

pub fn run() {
    let first = part1("data/8");
    println!("{}", first);
    let second = part2("data/8");
    println!("{}", second);
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let occurences: usize = s
        .trim()
        .lines()
        .map(|l| {
            l.split('|')
                .last()
                .unwrap()
                .split_whitespace()
                .fold(0, |acc, x| match x.len() {
                    2 | 4 | 3 | 7 => acc + 1,
                    _ => acc,
                })
        })
        .sum();
    occurences as i32
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<i32> = s.lines().map(|l| decode_output(String::from(l))).collect();
    lines.iter().sum()
}

fn decode_output(line: String) -> i32 {
    let output_codes: Vec<_> = line.split('|').last().unwrap().split_whitespace().collect();
    let mut input_codes: HashMap<_, i32> = line
        .split('|')
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| (s, -1))
        .collect();
    decode_input(&mut input_codes);
    let output_number: Vec<_> = output_codes
        .into_iter()
        .map(|x| get_number(x, &input_codes).to_string())
        .collect();
    let res = output_number.join("").parse::<i32>();

    res.unwrap()
}

fn get_number(code: &str, input_codes: &HashMap<&str, i32>) -> i32 {
    for (v, k) in input_codes.iter() {
        if code.chars().all(|x| String::from(*v).contains(x)) && code.len() == v.len() {
            return *k;
        }
    }
    0
}

fn decode_input(input_codes: &mut HashMap<&str, i32>) {
    let one = input_codes
        .keys()
        .filter(|x| x.len() == 2)
        .next()
        .unwrap()
        .clone();
    input_codes.insert(one, 1);
    let four = input_codes
        .keys()
        .filter(|x| x.len() == 4)
        .next()
        .unwrap()
        .clone();
    input_codes.insert(four, 4);
    let seven = input_codes
        .keys()
        .filter(|x| x.len() == 3)
        .next()
        .unwrap()
        .clone();
    input_codes.insert(seven, 7);
    let eight = input_codes
        .keys()
        .filter(|x| x.len() == 7)
        .next()
        .unwrap()
        .clone();
    input_codes.insert(eight, 8);

    let i = input_codes.clone();

    let len_six: Vec<_> = i
        .iter()
        .filter(|(k, _)| k.len() == 6)
        .map(|(k, _)| k)
        .collect();

    let six: Vec<_> = len_six
        .iter()
        .filter(|x| !first_contains_second(x, one))
        .collect();
    input_codes.insert(six[0], 6);
    let len_six: Vec<_> = len_six
        .iter()
        .filter(|k| k.len() == 6 && *k != six[0])
        .collect();

    let zero: Vec<_> = len_six
        .iter()
        .filter(|x| !first_contains_second(x, four))
        .collect();
    input_codes.insert(zero[0], 0);

    let nine: Vec<_> = len_six
        .iter()
        .filter(|k| k.len() == 6 && **k != six[0] && *k != zero[0])
        .collect();
    input_codes.insert(nine[0], 9);

    let len_five: Vec<_> = i
        .iter()
        .filter(|(k, _)| k.len() == 5)
        .map(|(k, _)| k)
        .collect();

    let three: Vec<_> = len_five
        .iter()
        .filter(|x| first_contains_second(x, one))
        .collect();
    input_codes.insert(three[0], 3);
    let len_five: Vec<_> = len_five
        .iter()
        .filter(|k| k.len() == 5 && *k != three[0])
        .collect();

    let five: Vec<_> = len_five
        .iter()
        .filter(|x| first_contains_second(six[0], x))
        .collect();
    input_codes.insert(five[0], 5);

    let two: Vec<_> = len_five
        .iter()
        .filter(|k| k.len() == 5 && **k != three[0] && *k != five[0])
        .collect();
    input_codes.insert(two[0], 2);
}

fn first_contains_second(first: &str, second: &str) -> bool {
    let mut res = true;
    for c in second.chars() {
        if !String::from(first).contains(c) {
            res = false;
        }
    }
    res
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/8_easy"), 26);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/8_easy"), 61229);
}

#[test]
fn test_part2_super_easy() {
    assert_eq!(part2("data/8_super_easy"), 5353);
}

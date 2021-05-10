use std::collections::HashMap;
use std::fs;

pub fn run() {
    let first = part1("data/10");
    println!("{}", first);
    let second = part2("data/10");
    println!("{}", second);
}

fn part1(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let mut numbers: Vec<i64> = s.lines().map(|x| x.parse().unwrap()).collect();
    numbers.push(0); // Starting outlet
    numbers.sort();
    let mut one = 0;
    let mut three = 1; // device is internally 3

    for i in 0..(numbers.len() - 1) {
        match numbers[i + 1] - numbers[i] {
            1 => one += 1,
            3 => three += 1,
            _ => panic!(
                "difference is not one through three {} {}",
                numbers[i + 1],
                numbers[i]
            ),
        };
    }
    one * three
}

fn calc_combinations(vector: &Vec<i64>, index: usize, map: &mut HashMap<i64, i64>) -> i64 {
    let length = vector.len();
    if index == length - 1 {
        return 1;
    } else if index >= length {
        return 0;
    }
    let value = vector[index];
    let mut combinations = 0;
    for i in index + 1..=index + 3 {
        if i < length && vector[i] - value <= 3 {
            let r;
            if let Some(x) = map.get(&(i as i64)) {
                r = *x;
            } else {
                r = calc_combinations(&vector, i, map);
                map.insert(i as i64, r);
            }
            combinations += r;
        }
    }
    combinations
}

fn part2(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let mut numbers: Vec<i64> = s.lines().map(|x| x.parse().unwrap()).collect();
    numbers.push(0); // Starting outlet
    numbers.sort();
    let mut map: HashMap<i64, i64> = HashMap::new();

    calc_combinations(&numbers, 0, &mut map)
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/10_easy"), 35);
}

#[test]
fn test_part1_medium() {
    assert_eq!(part1("data/10_medium"), 220);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/10_easy"), 8);
}

#[test]
fn test_part2_medium() {
    assert_eq!(part2("data/10_medium"), 19208);
}

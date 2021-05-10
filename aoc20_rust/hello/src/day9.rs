use std::fs;

pub fn run() {
    let first = part1("data/9", 25);
    println!("{}", first);
    let second = part2("data/9", 26796446);
    println!("{}", second);
}

fn validate(index: usize, preamble: usize, numbers: &Vec<i64>) -> bool {
    let sum = numbers[index];
    for i in (index - preamble)..index {
        for j in (index + 1 - preamble)..index {
            if i == j {
                continue;
            }
            if numbers[i] + numbers[j] == sum {
                return true;
            }
        }
    }
    false
}

fn part1(input: &str, preamble: usize) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let numbers: Vec<i64> = s.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    for i in preamble..numbers.len() {
        let valid = validate(i, preamble, &numbers);
        if !valid {
            return numbers[i];
        }
    }
    -1
}

fn part2(input: &str, sum: i64) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    
    let numbers: Vec<i64> = s.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    for i in 0..numbers.len() {
        let mut temp_sum = 0;
        let mut smallest = i64::MAX;
        let mut largest = i64::MIN;
        for j in i..numbers.len() {
            if numbers[j] > largest {
                largest = numbers[j];
            }
            if numbers[j] < smallest {
                smallest = numbers[j];
            }
            temp_sum += numbers[j];
            if temp_sum > sum {
                break;
            } else if temp_sum == sum {
                return smallest + largest;
            }
        }
    }
    -1
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/9_easy", 5), 127);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/9_easy", 127), 62);
}

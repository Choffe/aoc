use std::fs;

pub fn run() {
    let first = part1("data/3");
    println!("{}", first);
    let second = part2("data/3");
    println!("{}", second);
}

fn calc_occurenses(entries: &Vec<Vec<char>>) -> (Vec<i32>, Vec<i32>) {
    let length = entries[0].len();
    let mut ones = vec![0; length];
    let mut zeroes = vec![0; length];
    let lines: Vec<&char> = entries.iter().flatten().collect();
    for i in 0..lines.len() {
        match lines[i] {
            '0' => zeroes[i % length] += 1,
            '1' => ones[i % length] += 1,
            _ => panic!("Unsupported input"),
        };
    }
    (zeroes, ones)
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let length = s.trim().split('\n').next().unwrap().len();
    let lines: Vec<Vec<char>> = s.trim().split('\n').map(|l| l.chars().collect()).collect();

    let (zeroes, ones) = calc_occurenses(&lines);
    let mut eps = String::from("");
    let mut gamma = String::from("");
    for i in 0..length {
        if ones[i] > zeroes[i] {
            gamma.push('1');
            eps.push('0');
        } else {
            gamma.push('0');
            eps.push('1');
        }
    }
    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let eps = i32::from_str_radix(&eps, 2).unwrap();
    gamma * eps
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let length = s.trim().split('\n').next().unwrap().len();
    let mut oxy: Vec<Vec<char>> = s.trim().split('\n').map(|l| l.chars().collect()).collect();
    let mut co2 = oxy.clone();

    for i in 0..length {
        let (oxy_zeroes, oxy_ones) = calc_occurenses(&oxy);
        let (co2_zeroes, co2_ones) = calc_occurenses(&co2);

        if oxy_ones[i] >= oxy_zeroes[i] && oxy.len() > 1 {
            oxy = oxy.into_iter().filter(|word| word[i] == '1').collect();
        } else if oxy.len() > 1 {
            oxy = oxy.into_iter().filter(|word| word[i] == '0').collect();
        }
        if co2_ones[i] >= co2_zeroes[i] && co2.len() > 1 {
            co2 = co2.into_iter().filter(|word| word[i] == '0').collect();
        } else if co2.len() > 1 {
            co2 = co2.into_iter().filter(|word| word[i] == '1').collect();
        }
    }
    let oxy: String = oxy[0].iter().collect();
    let co2: String = co2[0].iter().collect();
    let oxy = i32::from_str_radix(&oxy, 2).unwrap();
    let co2 = i32::from_str_radix(&co2, 2).unwrap();
    oxy * co2
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/3_easy"), 198);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/3_easy"), 230);
}

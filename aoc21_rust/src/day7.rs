use std::fs;

pub fn run() {
    let first = part1("data/7");
    println!("{}", first);
    let second = part2("data/7");
    println!("{}", second);
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<i32> = s
        .trim()
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();
    let min_pos = lines.iter().min().unwrap();
    let max_pos = lines.iter().max().unwrap();
    let mut min_fuel = i32::MAX;

    for pos in (*min_pos as usize)..=(*max_pos as usize) {
        let fuel = lines.iter().fold(0, |acc, x| acc + (pos as i32 - *x).abs());
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
}

fn fuel_for_steps(steps: i32) -> i32 {
    let steps: f32 = steps as f32;
    ((steps / 2.0) * (steps + 1.0)) as i32
}

fn get_fuel_cost(start: i32, dest: i32) -> i32 {
    let diff = (dest - start).abs();
    fuel_for_steps(diff)
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<i32> = s
        .trim()
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();
    let min_pos = lines.iter().min().unwrap();
    let max_pos = lines.iter().max().unwrap();
    let mut min_fuel = i32::MAX;

    for pos in (*min_pos as usize)..=(*max_pos as usize) {
        let fuel = lines
            .iter()
            .fold(0, |acc, x| acc + get_fuel_cost(pos as i32, *x));
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/7_easy"), 37);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/7_easy"), 168);
}

#[test]
fn test_fuel_for_steps() {
    assert_eq!(fuel_for_steps(0), 0);
    assert_eq!(fuel_for_steps(1), 1);
    assert_eq!(fuel_for_steps(2), 3);
    assert_eq!(fuel_for_steps(3), 6);
    assert_eq!(fuel_for_steps(4), 10);
}

use std::fs;

pub fn run() {
    let first = part1("data/3");
    println!("{}", first);
    let second = part2("data/3");
    println!("{}", second);
}

fn part1(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = s.split('\n').collect();
    let mut map: Vec<Vec<char>> = vec!();
    for line in lines {
        let mut row: Vec<char> = vec!();
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }
    calc_trees(&map, 3, 1)
}

fn part2(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = s.split('\n').collect();
    let mut map: Vec<Vec<char>> = vec!();
    for line in lines {
        let mut row: Vec<char> = vec!();
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }
    calc_trees(&map, 1, 1) * calc_trees(&map, 3, 1) * calc_trees(&map, 5, 1) * calc_trees(&map, 7, 1) * calc_trees(&map, 1, 2)
}

fn calc_trees(map: &Vec<Vec<char>>, angle: usize, step: usize) -> i64 {
    let mut number_trees = 0;
    let mut i = 0;
    for j in (0..map.len()).step_by(step) {
        if map[j][i % map[j].len()] == '#' {
            number_trees += 1;
        }
        i = (i + angle) % map[j].len();
    }
    number_trees
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/3_easy"), 7);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/3_easy"), 336);
}

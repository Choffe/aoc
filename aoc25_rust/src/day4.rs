use std::fs;

const INPUT: &str = "data/4/input";
const TEST_INPUT: &str = "data/4/test";

fn check_neighbours(map: &[Vec<bool>], row: usize, col: usize) -> usize {
    let mut neighbour_rolls = 0;
    for i in [-1, 0, 1] {
        for j in [-1, 0, 1] {
            if j == 0 && i == 0 {
                continue;
            }
            if map
                .get((row as isize + i) as usize)
                .and_then(|r| r.get((col as isize + j) as usize))
                .copied()
                .unwrap_or(false)
            {
                neighbour_rolls += 1;
            }
        }
    }
    neighbour_rolls
}

fn part1(input: &str) -> u32 {
    let s = fs::read_to_string(input).unwrap();

    let map: Vec<Vec<bool>> = s
        .trim()
        .lines()
        .map(|x| x.chars().map(|y| y == '@').collect())
        .collect();

    let mut number_free_rolls = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] && check_neighbours(&map, row, col) < 4 {
                number_free_rolls += 1;
            };
        }
    }
    number_free_rolls
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();

    let mut map: Vec<Vec<bool>> = s
        .trim()
        .lines()
        .map(|x| x.chars().map(|y| y == '@').collect())
        .collect();

    let mut last_number_free_rolls = -1;
    let mut number_free_rolls = 0;
    while last_number_free_rolls != number_free_rolls {
        last_number_free_rolls = number_free_rolls;

        for row in 0..map.len() {
            for col in 0..map[0].len() {
                if map[row][col] && check_neighbours(&map, row, col) < 4 {
                    number_free_rolls += 1;
                    map[row][col] = false;
                };
            }
        }
    }
    number_free_rolls
}

pub fn run() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 13);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 43);
}

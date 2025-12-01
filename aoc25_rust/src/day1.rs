use std::fs;
use std::str::FromStr;

const INPUT: &str = "data/1/input";
const TEST_INPUT: &str = "data/1/test";

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    dist: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = get_direction(s);
        let dist = get_distance(s);

        Ok(Instruction { dir, dist })
    }
}

fn get_first_char(line: &str) -> char {
    line.chars().next().unwrap_or('N')
}

fn get_direction(line: &str) -> Direction {
    match get_first_char(line) {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("No direction on this line {}", line),
    }
}

fn get_distance(line: &str) -> i32 {
    let (_, distance) = line.split_at(
        line.char_indices()
            .nth(1)
            .map(|(i, _)| i)
            .unwrap_or(line.len()),
    );

    distance.parse().unwrap()
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let instructions: Vec<Instruction> = s.trim().split('\n').map(|x| x.parse().unwrap()).collect();
    let mut index = 50;
    let mut times_at_zero = 0;

    for instruction in &instructions {
        match &instruction.dir {
            Direction::Left => {
                index -= instruction.dist;
                while index < 0 {
                    index += 100;
                }
            }
            Direction::Right => index = (index + instruction.dist) % 100,
        };
        if index == 0 {
            times_at_zero += 1;
        }
    }
    times_at_zero
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let instructions: Vec<Instruction> = s.trim().split('\n').map(|x| x.parse().unwrap()).collect();
    let mut index = 50;
    let mut times_at_zero = 0;

    for instruction in &instructions {
        for _ in 0..instruction.dist {
            index = match &instruction.dir {
                Direction::Left => (index - 1i32).rem_euclid(100),
                Direction::Right => (index + 1i32).rem_euclid(100),
            };

            if index == 0 {
                times_at_zero += 1;
            }
        }
        // println!("{:?} {} {}", instruction, index, times_at_zero);
    }

    times_at_zero
}

pub fn run() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 3);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 6);
}

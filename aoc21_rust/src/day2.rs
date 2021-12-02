use std::fs;

pub fn run() {
    let first = part1("data/2");
    println!("{}", first);
    let second = part2("data/2");
    println!("{}", second);
}

struct Position {
    depth: i32,
    horizontal: i32,
}

struct SubmarineState {
    aim: i32,
    depth: i32,
    horizontal: i32,
}

#[derive(Debug)]
enum Instruction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn generate_instructions(input: &String) -> Vec<Instruction> {
    input
        .trim()
        .split('\n')
        .map(|row| {
            let row: Vec<_> = row.split(' ').collect();
            match row[0] {
                "up" => Instruction::Up(row[1].parse().unwrap()),
                "down" => Instruction::Down(row[1].parse().unwrap()),
                "forward" => Instruction::Forward(row[1].parse().unwrap()),
                _ => panic!("Unsupported instruction"),
            }
        })
        .collect()
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines = generate_instructions(&s);

    let mut my_pos = Position {
        depth: 0,
        horizontal: 0,
    };
    for movement in lines {
        match movement {
            Instruction::Up(v) => my_pos.depth -= v,
            Instruction::Down(v) => my_pos.depth += v,
            Instruction::Forward(v) => my_pos.horizontal += v,
        };
    }
    my_pos.depth * my_pos.horizontal
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines = generate_instructions(&s);

    let mut my_state = SubmarineState {
        aim: 0,
        depth: 0,
        horizontal: 0,
    };
    for movement in lines {
        match movement {
            Instruction::Up(v) => my_state.aim -= v,
            Instruction::Down(v) => my_state.aim += v,
            Instruction::Forward(v) => {
                my_state.horizontal += v;
                my_state.depth += v * my_state.aim;
            }
        };
    }
    my_state.depth * my_state.horizontal
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/2_easy"), 150);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/2_easy"), 900);
}

use std::fs;

pub fn run() {
    let first = part1("data/8");
    println!("{}", first);
    let second = part2("data/8");
    println!("{}", second);
}

fn part1(input: &str) -> usize {
    let s = fs::read_to_string(input).unwrap();
    let program: Vec<&str> = s.lines().collect();
    let mut pc = 0;
    let mut visited = vec![];
    let mut acc = 0;

    loop {
        if visited.contains(&pc) {
            break;
        }
        visited.push(pc);
        let (op, value) = program[pc].split_at(3);
        let (sign, value) = value.split_at(2);
        let sign = sign.trim();
        let value = value.parse::<usize>().unwrap();
        match op {
            "nop" => (),
            "acc" => {
                if sign == "+" {
                    acc += value;
                } else {
                    acc -= value;
                }
            },
            "jmp" => {
                if sign == "+" {
                    pc += value;
                } else {
                    pc -= value;
                }
                continue;
            }
            _ => panic!("Unknown operation")
        };
        
        pc += 1;
    }
    acc
}


fn part2(input: &str) -> usize {
    let s = fs::read_to_string(input).unwrap();
    let org_program: Vec<String> = s.lines().map(|x| String::from(x)).collect();
    let mut swap_index = 0;
    loop {
        let mut pc = 0;
        let mut visited = vec![];
        let mut acc: i32 = 0;
        let mut program = org_program.clone();
        for i in swap_index..program.len() {
            let (op, _) = program[i].split_at(3);
            if op == "nop" {
                program[i] = program[i].replace("nop", "jmp");
                swap_index = i + 1;
                break;
            } else if op == "jmp" {
                program[i] = program[i].replace("jmp", "nop");
                swap_index = i + 1;
                break;
            }
        }
        loop {
            if pc >= program.len() {
                return acc as usize;
            }
            if visited.contains(&pc) {
                break;
            }
            visited.push(pc);
            let (op, value) = program[pc].split_at(3);
            let (sign, value) = value.split_at(2);
            let sign = sign.trim();
            let value = value.parse::<usize>().unwrap();
            match op {
                "nop" => (),
                "acc" => {
                    if sign == "+" {
                        acc += value as i32;
                    } else {
                        acc -= value as i32;
                    }
                },
                "jmp" => {
                    if sign == "+" {
                        pc += value;
                    } else {
                        pc -= value;
                    }
                    continue;
                }
                _ => panic!("Unknown operation")
            };
            
            pc += 1;
        }
    }
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/8_easy"), 5);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/8_easy"), 8);
}

use std::collections::HashMap;
use std::fs;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    println!("intersection dist {}", solve(1));
}

fn part2() {
    println!("intersection timing dist {}", solve(2));
}

fn solve(part: i32) -> i32 {
    let s = fs::read_to_string("data/2019/3").expect("Supply data file");
    let it = s.split('\n');
    // dict contains key (x,y) with data(steps for first_wire)
    let mut dict = HashMap::new();
    let mut intersect_dist = i32::MAX;
    let mut wire_index = 0;
    for wire in it {
        let wire = wire.split(',');
        let mut current_pos = (0, 0);
        dict.insert(current_pos, (wire_index, 0));
        let mut number_of_steps = 1;
        for e in wire {
            let (dir, value) = match e.chars().next() {
                Some(c) => e.split_at(c.len_utf8()),
                None => panic!("Row without anything"),
            };
            let value: i32 = value.parse().unwrap();
            for _ in 0..value {
                if dir == "U" {
                    current_pos.1 += 1;
                } else if dir == "D" {
                    current_pos.1 -= 1;
                } else if dir == "L" {
                    current_pos.0 -= 1;
                } else if dir == "R" {
                    current_pos.0 += 1;
                }
                if dict.contains_key(&current_pos) {
                    let (wire, steps_to) = dict.get(&current_pos).unwrap();
                    if wire != &wire_index {
                        let dist = match part {
                            1 => i32::abs(current_pos.0) + i32::abs(current_pos.1),
                            2 => steps_to + number_of_steps,
                            _ => panic!("May only solve part 1 or 2"),
                        };
                        if dist < intersect_dist {
                            intersect_dist = dist;
                        }
                    }
                }
                dict.insert(current_pos, (wire_index, number_of_steps));
                number_of_steps += 1;
            }
        }
        wire_index += 1;
    }

    intersect_dist
}

use std::fs;

const MAGIC_NUMBER: i32 = 2020;

pub fn run() {
    let first = part1("data/1");
    println!("{}", first);
    
    let second = part2("data/1");
    println!("{}", second);
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let list: Vec<i32> = s.split('\n').map(|x| x.parse::<i32>().unwrap()).collect();
    for i in 0..list.len() {
        for j in i..list.len() {
            if list[i] + list[j] == MAGIC_NUMBER {
                return list[i] * list[j];
            }
        }
    }
    panic!("Can't find such a number");
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let list: Vec<i32> = s.split('\n').map(|x| x.parse::<i32>().unwrap()).collect();
    for i in 0..list.len() {
        for j in i..list.len() {
            for k in j..list.len() {
                if list[i] + list[j] + list[k] == MAGIC_NUMBER {
                    return list[i] * list[j] * list[k];
                }
            }
        }
    }
    panic!("Can't find such a number");
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/1_easy"), 514579);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/1_easy"), 241861950);
}

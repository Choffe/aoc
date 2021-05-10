use std::fs;
use regex::Regex;
use std::collections::HashMap;

pub fn run() {
    let first = part1("data/14");
    println!("{}", first);
    let second = part2("data/14");
    println!("{}", second);
}

fn mask_value(mask: &str, value: i64) -> i64 {
    let mut new_value = value;
    for (pos, ch) in mask.chars().enumerate() {
        let position_value = (2 as i64).pow((35 - pos) as u32);
        match ch {
            '0' => new_value &= !position_value,
            '1' => new_value |= position_value,
            'X' => (),
            _ => panic!("{} character should not be in the mask", ch)
        }
    }
    new_value
}

fn part1(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    //mem[62085] = 231745
    let re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    let mut map: HashMap<usize, i64> = HashMap::new();
    let mut current_mask: &str = "0";
    for line in s.lines() {
        let instruction: Vec<&str> = line.split(" = ").collect();
        if instruction.len() != 2 {
            panic!("Should have an op and value");
        }
        match instruction[0] {
            "mask" => {
                current_mask = instruction[1];
            },
            _ => {
                let value;
                let address;
                match re.captures(line) {
                    Some(cap) => {
                        address = cap[1].parse::<usize>().unwrap();
                        value = cap[2].parse::<i64>().unwrap();
                    }
                    None => panic!("Couldn't capture memory, {}", line),
                }
                let value = mask_value(&current_mask, value);
                map.insert(address, value);
            }
        }
    }
    map.iter().fold(0, |acc, (_, v)| acc + v)
}

fn mask_addresses(index: usize, mut mask: Vec<char>, value: i64) -> Vec<i64> {
    let mut res = vec![];
    if index >= mask.len() {
        res.push(i64::from_str_radix(&mask.iter().collect::<String>(), 2).unwrap());
        return res;
    }
    match mask[index] {
        '0' => {
            let position_value = (2 as i64).pow((35 - index) as u32);
            let v = 0 | position_value;
            mask[index] = if value & v != 0 {'1'} else {'0'};
            return mask_addresses(index+1, mask, value);
        },
        '1' => return mask_addresses(index+1, mask, value),
        'X' => {
            mask[index] = '0';
            res.append(&mut mask_addresses(index + 1, mask.clone(), value));
            mask[index] = '1';
            res.append(&mut mask_addresses(index + 1, mask, value));
            return res;
        },
        _ => panic!("this should not happend")
    }
}

fn part2(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    //mem[62085] = 231745
    let re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    let mut map: HashMap<usize, i64> = HashMap::new();
    let mut current_mask: &str = "0";
    for line in s.lines() {
        let instruction: Vec<&str> = line.split(" = ").collect();
        if instruction.len() != 2 {
            panic!("Should have an op and value");
        }
        match instruction[0] {
            "mask" => {
                current_mask = instruction[1];
            },
            _ => {
                let value;
                let address;
                match re.captures(line) {
                    Some(cap) => {
                        address = cap[1].parse::<i64>().unwrap();
                        value = cap[2].parse::<i64>().unwrap();
                    }
                    None => panic!("Couldn't capture memory, {}", line),
                }
                let masked_addresses = mask_addresses(0, current_mask.chars().collect::<Vec<char>>(), address);
                for masked_add in masked_addresses {
                    map.insert(masked_add as usize, value);
                }
            }
        }
    }
    map.iter().fold(0, |acc, (_, v)| acc + v)
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/14_easy"), 165);
}

#[test]
fn test_mask_value() {
    assert_eq!(mask_value("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 11), 73);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/14_medium"), 208);
}

#[test]
fn test_mask_address() {
    let list = mask_addresses(0, String::from("000000000000000000000000000000X1001X").chars().collect::<Vec<char>>(), 42);
    assert_eq!(list[0], 26);
    assert_eq!(list[1], 27);
    assert_eq!(list[2], 58);
    assert_eq!(list[3], 59);
}

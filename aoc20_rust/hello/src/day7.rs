use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct BagInfo {
    color: String,
}

fn create_bag_info(text: &str, pattern: &regex::Regex) -> (BagInfo, Vec<(i32, BagInfo)>) {
    let cap = pattern.captures(text).unwrap();
    let mut v = vec![];
    let bag = match cap.get(1) {
        Some(x) => BagInfo {
            color: String::from(x.as_str()),
        },
        None => panic!("Can't parse inital bag info"),
    };
    for i in vec![4, 7, 10, 13] {
        match cap.get(i) {
            Some(x) => {
                match cap.get(i - 1) {
                    Some(y) => v.push((
                        y.as_str().parse::<i32>().unwrap(),
                        BagInfo {
                            color: String::from(x.as_str()),
                        },
                    )),
                    None => println!("Couldn't parse number for this bag {:?}", x),
                };
            }
            None => (),
        };
    }
    (bag, v)
}

fn contains_bag(bag: &BagInfo, needle_bag: &BagInfo, map: &HashMap<BagInfo, Vec<(i32, BagInfo)>>) -> bool {
    if bag == needle_bag {
        return false;
    }
    let list = map.get(&bag).unwrap().to_vec();
    if list.iter().any(|(_, x)| x == needle_bag) {
        return true;
    }
    for (_, containing_bag) in list {
        if contains_bag(&containing_bag, needle_bag, map) {
            return true;
        }
    }
    false
}

fn count_bags(bag: &BagInfo, map: &HashMap<BagInfo, Vec<(i32, BagInfo)>>) -> i32 {
    let mut sum = 1;
    let list = map.get(&bag).unwrap().to_vec();
    for (n, contained_bag) in list {
        sum += n * count_bags(&contained_bag, map);
    }
    sum
}

pub fn run() {
    let first = part1("data/7");
    println!("{}", first);
    let second = part2("data/7");
    println!("{}", second);
}

fn part1(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let mut all_bags: HashMap<BagInfo, Vec<(i32, BagInfo)>> = HashMap::new();
    let pattern = Regex::new(r"^(\w* \w*) \w* contain( (\d+) (\w* \w*) \w*[,.])?( (\d+) (\w* \w*) \w*[,.])?( (\d+) (\w* \w*) \w*[,.])?( (\d+) (\w* \w*) \w*[,.])?").unwrap();
    for line in s.lines() {
        let (b, c) = create_bag_info(line, &pattern);
        all_bags.insert(b, c);
    }
    let needle = BagInfo {
        color: String::from("shiny gold"),
    };
    let mut sum = 0;
    for (b, _) in all_bags.iter() {
        if contains_bag(b, &needle, &all_bags) {
            sum += 1;
        }
    }
    sum
}


fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let mut all_bags: HashMap<BagInfo, Vec<(i32, BagInfo)>> = HashMap::new();
    let pattern = Regex::new(r"^(\w* \w*) \w* contain( (\d+) (\w* \w*) \w*[,.])?( (\d+) (\w* \w*) \w*[,.])?( (\d+) (\w* \w*) \w*[,.])?( (\d+) (\w* \w*) \w*[,.])?").unwrap();
    for line in s.lines() {
        let (b, c) = create_bag_info(line, &pattern);
        all_bags.insert(b, c);
    }
    let needle = BagInfo {
        color: String::from("shiny gold"),
    };
    let mut sum = 0;
    for (b, _) in all_bags.iter() {
        if b == &needle {
            sum += count_bags(b, &all_bags);
        }
    }
    sum - 1
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/7_easy"), 4);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/7_easy"), 32);
}

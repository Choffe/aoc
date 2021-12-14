use std::collections::HashMap;
use std::fs;

pub fn run() {
    let first = part1("data/14");
    println!("{}", first);
    let second = part2("data/14");
    println!("{}", second);
}

fn part1(input: &str) -> i32 {
    polymerization(input, 10)
}

fn part2(input: &str) -> i128 {
    polymerization_2(input, 40)
}

fn polymerization_2(input: &str, steps: usize) -> i128 {
    let s = fs::read_to_string(input).unwrap();
    let mut lines = s.lines();
    let polymer: Vec<_> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut pair: HashMap<(char, char), i128> = HashMap::new();
    for i in 1..polymer.len() {
        let f = polymer.get(i - 1).unwrap();
        let s = polymer.get(i).unwrap();
        if let Some(entry) = pair.get_mut(&(*f, *s)) {
            *entry += 1;
        } else {
            pair.insert((*f, *s), 1);
        }
    }

    let mut rules: HashMap<(char, char), char> = HashMap::new();
    for line in lines {
        let mut split = line.split(" -> ");
        let from = split.next().unwrap();
        let to = split.next().unwrap();
        let first = from.chars().nth(0).unwrap();
        let second = from.chars().nth(1).unwrap();

        rules.insert((first, second), to.chars().nth(0).unwrap());
    }
    println!("rules {:?}", rules);

    println!("pair {:?}", pair);
    for _ in 0..steps {
        let mut new: HashMap<(char, char), i128> = HashMap::new();
        for ((first, second), to) in rules.iter() {
            if let Some(v) = pair.remove(&(*first, *second)) {
                *new.entry((*first, *to)).or_insert(0) += v;
                *new.entry((*to, *second)).or_insert(0) += v;
            }
        }
        for ((first, second), v) in new {
            *pair.entry((first, second)).or_insert(0) += v;
        }
    }
    println!("pair {:?}", pair);

    let mut occurencies: HashMap<_, _> = HashMap::new();
    pair.iter().for_each(|((first, second), v)| {
        let entry = occurencies.entry(first).or_insert(0);
        *entry += v;
        let entry = occurencies.entry(second).or_insert(0);
        *entry += v;
    });
    *occurencies
        .entry(polymer.iter().nth(0).unwrap())
        .or_insert(0) += 1;
    *occurencies
        .entry(polymer.iter().nth(polymer.len() - 1).unwrap())
        .or_insert(0) += 1;
    let mut letters: Vec<(_, _)> = occurencies.into_iter().collect();
    //before sorting add first and last letter in polymer first
    //and let high/2 and low/2 later
    letters.sort_by(|(_, a), (_, b)| a.cmp(b));
    println!("letters {:?}", letters);
    let (_, high) = letters.last().unwrap();
    let (_, low) = letters.first().unwrap();
    high / 2 - low / 2
}

fn polymerization(input: &str, steps: usize) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let mut lines = s.lines();
    let mut polymer = String::from(lines.next().unwrap());
    lines.next();
    let map: HashMap<_, _> = lines
        .map(|l| {
            let mut split = l.split(" -> ");
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect();

    for _ in 0..steps {
        polymer = extend_polymer(polymer, &map);
    }
    let mut occurencies: HashMap<_, _> = HashMap::new();
    polymer.chars().for_each(|c| {
        let entry = occurencies.entry(c).or_insert(0);
        *entry += 1;
    });
    let mut letters: Vec<(_, _)> = occurencies.into_iter().collect();
    letters.sort_by(|(_, a), (_, b)| a.cmp(b));
    let (_, high) = letters.last().unwrap();
    let (_, low) = letters.first().unwrap();
    high - low
}

fn extend_polymer(polymer: String, map: &HashMap<&str, &str>) -> String {
    let mut new_poly = vec![];
    for i in 0..polymer.len() - 1 {
        let pair = polymer.get(i..=i + 1).unwrap();
        new_poly.push(pair.chars().nth(0).unwrap());
        if let Some(insert) = map.get(pair) {
            new_poly.push(insert.chars().nth(0).unwrap());
        }
    }
    new_poly.push(polymer.chars().last().unwrap());

    new_poly.into_iter().collect()
}
#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/14_easy"), 1588);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/14_easy"), 2188189693529);
}

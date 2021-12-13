use std::collections::HashMap;
use std::fs;

pub fn run() {
    let first = part1("data/12");
    println!("{}", first);
    let second = part2("data/12");
    println!("{}", second);
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    s.lines().for_each(|l| {
        let mut split = l.split("-");
        let from = split.next().unwrap();
        let to = split.next().unwrap();
        if let Some(entry) = map.get_mut(from) {
            entry.push(to);
        } else {
            map.insert(from, vec![to]);
        }
        if let Some(entry) = map.get_mut(to) {
            entry.push(from);
        } else {
            map.insert(to, vec![from]);
        }
    });

    let paths = find_paths("start", vec![], &map);

    paths.len() as i32
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    s.lines().for_each(|l| {
        let mut split = l.split("-");
        let from = split.next().unwrap();
        let to = split.next().unwrap();
        if let Some(entry) = map.get_mut(from) {
            entry.push(to);
        } else {
            map.insert(from, vec![to]);
        }
        if let Some(entry) = map.get_mut(to) {
            entry.push(from);
        } else {
            map.insert(to, vec![from]);
        }
    });

    let paths = find_paths_twice("start", vec![], &map, false);

    paths.len() as i32
}

fn find_paths<'a>(
    cave: &'a str,
    history: Vec<&'a str>,
    connections: &'a HashMap<&str, Vec<&str>>,
) -> Vec<Vec<&'a str>> {
    let mut paths = vec![];
    if cave == "end" {
        let mut this_path = history.clone();
        this_path.push(cave);
        paths.push(this_path);
        return paths;
    }
    let neighbours = connections.get(cave).unwrap();
    for neighbour in neighbours {
        if is_big(neighbour) || !history.contains(neighbour) {
            let mut one_path = history.clone();
            one_path.push(cave);
            paths.append(&mut find_paths(neighbour, one_path, &connections));
        }
    }
    paths
}

fn find_paths_twice<'a>(
    cave: &'a str,
    history: Vec<&'a str>,
    connections: &'a HashMap<&str, Vec<&str>>,
    visited_small_twice: bool,
) -> Vec<Vec<&'a str>> {
    let mut paths = vec![];
    if cave == "end" {
        let mut this_path = history.clone();
        this_path.push(cave);
        paths.push(this_path);
        return paths;
    }
    let neighbours = connections.get(cave).unwrap();
    for neighbour in neighbours {
        if visited_small_twice {
            if is_big(neighbour) || !history.contains(neighbour) {
                let mut one_path = history.clone();
                one_path.push(cave);
                paths.append(&mut find_paths_twice(
                    neighbour,
                    one_path,
                    &connections,
                    true,
                ));
            }
        } else {
            if is_big(neighbour) || !history.contains(neighbour) {
                let mut one_path = history.clone();
                one_path.push(cave);
                paths.append(&mut find_paths_twice(
                    neighbour,
                    one_path,
                    &connections,
                    false,
                ));
            } else if *neighbour != "start" {
                let mut one_path = history.clone();
                one_path.push(cave);
                paths.append(&mut find_paths_twice(
                    neighbour,
                    one_path,
                    &connections,
                    true,
                ));
            }
        }
    }
    paths
}
fn is_big(cave: &str) -> bool {
    cave.chars().next().unwrap().is_uppercase()
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/12_easy"), 10);
}

#[test]
fn test_part1_medium() {
    assert_eq!(part1("data/12_medium"), 19);
}

#[test]
fn test_part1_hard() {
    assert_eq!(part1("data/12_hard"), 226);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/12_easy"), 36);
}

#[test]
fn test_part2_medium() {
    assert_eq!(part2("data/12_medium"), 103);
}

#[test]
fn test_part2_hard() {
    assert_eq!(part2("data/12_hard"), 3509);
}

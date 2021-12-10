use std::fs;

pub fn run() {
    let first = part1("data/10");
    println!("{}", first);
    let second = part2("data/10");
    println!("{}", second);
}

fn matches(open: &char, close: &char) -> bool {
    *open == '{' && *close == '}'
        || *open == '(' && *close == ')'
        || *open == '<' && *close == '>'
        || *open == '[' && *close == ']'
}

fn value(bracket: &char) -> i32 {
    match *bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        c => panic!("Can't get value from char {}", c),
    }
}

fn points(line: &str) -> Option<i32> {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '{' | '(' | '<' | '[' => stack.push(c),
            '}' | ')' | '>' | ']' => {
                let o;
                match stack.pop() {
                    Some(x) => o = x,
                    None => panic!("No more in stack"),
                };
                if !matches(&o, &c) {
                    return Some(value(&c));
                }
            }
            _ => panic!("Unhandled char"),
        };
    }
    if stack.len() != 0 {
        return None;
    }
    Some(0)
}

fn autocomplete_score(line: &str) -> i128 {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '{' | '(' | '<' | '[' => stack.push(c),
            '}' | ')' | '>' | ']' => {
                stack.pop();
                ()
            }
            _ => panic!("Unhandled char"),
        };
    }
    stack
        .iter()
        .map(|c| value_of_matching(c))
        .rev()
        .fold(0, |acc, v| (acc * 5) + v)
}

fn value_of_matching(bracket: &char) -> i128 {
    match bracket {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("Unable to score matching bracket"),
    }
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    s.lines().filter_map(|l| points(l)).sum()
}

fn part2(input: &str) -> i128 {
    let s = fs::read_to_string(input).unwrap();
    let mut scores: Vec<_> = s
        .lines()
        .filter(|l| points(l).is_none())
        .map(|l| autocomplete_score(l))
        .collect();

    scores.sort();
    let middle = scores[scores.len() / 2];
    middle
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/10_easy"), 26397);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/10_easy"), 288957);
}

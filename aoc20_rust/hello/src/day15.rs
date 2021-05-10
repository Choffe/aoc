use std::fs;
use std::collections::HashMap;

pub fn run() {
    let first = part1("data/15");
    println!("{}", first);
    let second = part2("data/15");
    println!("{}", second);
}


fn part1(input: &str) -> i64 {
    nth_spoken_value(input, 2020)
}

fn nth_spoken_value(input: &str, n: usize) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let mut map: HashMap<i64, usize> = HashMap::new();
    let mut turn: usize = 1;
    let mut last_spoken = None;

    for value in s.split(",") {
        let value = value.parse::<i64>().unwrap();
        last_spoken = map.insert(value, turn);
        turn += 1;
    }
    let mut next_spoken = 0;
    while turn <= n {
        next_spoken = match last_spoken {
            Some(old_turn) => (turn - 1) - old_turn,
            None => 0
        };
        last_spoken = map.insert(next_spoken as i64, turn);
        turn += 1;
    }
    next_spoken as i64
}

fn part2(input: &str) -> i64 {
    nth_spoken_value(input, 30000000)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_easy() {
        assert_eq!(part1("data/15_easy"), 436);
    }
}


use std::fs;

pub fn run() {
    let first = part1("data/6");
    println!("{}", first);
    let second = part2("data/6");
    println!("{}", second);
}

struct Fish {
    birth_in: i32,
}

impl Fish {
    fn step(&mut self) -> Option<Fish> {
        if self.birth_in == 0 {
            self.birth_in = 6;
            return Some(Fish { birth_in: 8 });
        } else {
            self.birth_in -= 1;
        }
        None
    }
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let mut fishes: Vec<Fish> = s
        .trim()
        .split(',')
        .map(|x| Fish {
            birth_in: x.trim().parse().unwrap(),
        })
        .collect();
    let nbr_steps = 80;
    for _ in 0..nbr_steps {
        let mut new_fishes = vec![];
        for i in 0..fishes.len() {
            if let Some(fish) = fishes[i].step() {
                new_fishes.push(fish);
            }
        }
        fishes.append(&mut new_fishes);
    }
    fishes.len() as i32
}

#[derive(Debug, Copy, Clone)]
struct Day {
    adults: u128,
    childs: u128,
}

impl Day {
    fn new() -> Day {
        Day {
            adults: 0,
            childs: 0,
        }
    }
}
fn part2(input: &str) -> u128 {
    let s = fs::read_to_string(input).unwrap();
    let mut days = [Day::new(); 7];
    let init: Vec<usize> = s
        .trim()
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();

    for i in init {
        days[i].adults += 1;
    }
    for i in 0..256 {
        days[(i + 2) % days.len()].childs = days[i % days.len()].adults;
        days[i % days.len()].adults += days[i % days.len()].childs;
        days[i % days.len()].childs = 0;
    }
    days.iter().fold(0, |acc, d| acc + d.adults + d.childs)
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/6_easy"), 5934);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/6_easy"), 26984457539u128);
}

use std::collections::HashMap;
use std::fs;

pub fn run() {
    let first = part1("data/5");
    println!("{}", first);
    let second = part2("data/5");
    println!("{}", second);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Coord,
    end: Coord,
    coords: Vec<Coord>,
    is_straight: bool,
}

impl Line {
    fn new(start: Coord, end: Coord, consider_diagonal: bool) -> Line {
        let is_straight = start.x == end.x || start.y == end.y;
        let mut line = Line {
            start: start,
            end: end,
            coords: vec![],
            is_straight: is_straight,
        };
        line.populate_inbetween(consider_diagonal);
        line
    }

    fn populate_inbetween(&mut self, consider_diagonal: bool) {
        if !self.is_straight && !consider_diagonal {
            return;
        }
        let delta_x = self.end.x - self.start.x;
        let delta_y = self.end.y - self.start.y;
        let steps = std::cmp::max(delta_y.abs(), delta_x.abs());
        for i in 0..=steps {
            self.coords.push(Coord {
                x: self.start.x + (delta_x / steps) * i,
                y: self.start.y + (delta_y / steps) * i,
            });
        }
    }
}

fn solve(s: String, consider_diagonal: bool) -> usize {
    let lines: Vec<Line> = s
        .lines()
        .map(|l| {
            let pair: Vec<Coord> = l
                .split("->")
                .map(|x| {
                    let mut splitted = x.split(',');
                    let first = splitted.next().unwrap().trim().parse().unwrap();
                    let second = splitted.next().unwrap().trim().parse().unwrap();
                    Coord {
                        x: first,
                        y: second,
                    }
                })
                .collect();
            Line::new(pair[0], pair[1], consider_diagonal)
        })
        .collect();

    let mut map: HashMap<Coord, i32> = HashMap::new();
    for line in lines {
        if !line.is_straight && !consider_diagonal {
            continue;
        }
        line.coords.into_iter().for_each(|c| {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        });
    }
    map.into_iter().filter(|(_, v)| *v > 1).count()
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    solve(s, false) as i32
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    solve(s, true) as i32
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/5_easy"), 5);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/5_easy"), 12);
}

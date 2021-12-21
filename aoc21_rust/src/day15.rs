use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;

pub fn run() {
    let first = part1("data/15");
    println!("{}", first);
    let second = part2("data/15");
    println!("{}", second);
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let map: Vec<Vec<u32>> = s
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut risks = HashMap::new();
    let mut visited = vec![];
    let my_pos = (0, 0);

    let risk = calculate_risk_dfs(my_pos, &map, &mut risks, &mut visited);

    let risk = risk.unwrap() - map[0][0];

    risk as i32
}

fn part2(input: &str) -> u32 {
    let s = fs::read_to_string(input).unwrap();
    let map: Vec<Vec<u32>> = s
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let risks = create_large_map(map);
    calculate_risk_bfs(&risks)
}

fn create_large_map(small_map: Vec<Vec<u32>>) -> HashMap<(i32, i32), u32> {
    let mut risks_map = HashMap::new();
    for y in 0..small_map.len() * 5 {
        for x in 0..small_map.get(0).unwrap().len() * 5 {
            let x_index = x % small_map.get(0).unwrap().len();
            let y_index = y % small_map.len();
            let mut value =
                small_map[y_index][x_index] + (x / small_map.get(0).unwrap().len()) as u32;
            value += (y / small_map.len()) as u32;
            if value > 9 {
                value -= 9;
            }
            risks_map.insert((x as i32, y as i32), value);
        }
    }

    risks_map
}

fn calculate_risk_bfs(risks: &HashMap<(i32, i32), u32>) -> u32 {
    let neighbours = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    let mut currently_best = HashMap::new();
    let mut queue = BinaryHeap::from([(Reverse(0), 0, 0)]);
    while let Some((Reverse(risk), x, y)) = queue.pop() {
        let entry = currently_best.entry((x, y)).or_insert(u32::MAX);
        if risk < *entry {
            *entry = risk;
            for (nx, ny) in neighbours {
                if let Some(old_risk) = risks.get(&(x + nx, y + ny)) {
                    queue.push((Reverse(risk + old_risk), x + nx, y + ny));
                }
            }
        }
    }
    let max = currently_best.keys().max().unwrap();
    currently_best[max]
}

fn add_coord(
    first: (usize, usize),
    second: (i32, i32),
    map: &Vec<Vec<u32>>,
    visited: &Vec<(usize, usize)>,
) -> Option<(usize, usize)> {
    let x = first.0 as i32 + second.0;
    let y = first.1 as i32 + second.1;
    if visited.contains(&(x as usize, y as usize)) {
        return None;
    }
    if x < 0 || x >= map.get(0).unwrap().len() as i32 || y < 0 || y >= map.len() as i32 {
        None
    } else {
        Some((x as usize, y as usize))
    }
}

fn calculate_risk_dfs(
    coord: (usize, usize),
    map: &Vec<Vec<u32>>,
    risks: &mut HashMap<(usize, usize), u32>,
    visited: &mut Vec<(usize, usize)>,
) -> Option<u32> {
    visited.push(coord);
    let (x, y) = coord;
    if x == map.len() - 1 && y == map.len() - 1 {
        risks.insert(coord, map[y][x]);
        return Some(map[y][x]);
    }
    let neighbours = [(0, 1), (1, 0)];
    let neighbours: Vec<_> = neighbours
        .iter()
        .filter_map(|c| add_coord(coord, *c, map, visited))
        .collect();

    let mut min_risk = u32::MAX;
    for n in neighbours {
        let mut risk = u32::MAX;
        if let Some(known_risk) = risks.get(&n) {
            risk = *known_risk;
        } else {
            if let Some(way_out) = calculate_risk_dfs(n, map, risks, &mut visited.clone()) {
                risk = way_out;
            }
        }
        if risk < min_risk {
            min_risk = risk;
        }
    }
    if min_risk == u32::MAX {
        return None;
    }
    let risk = map[y][x] + min_risk;
    risks.insert(coord, risk);
    Some(risk)
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/15_easy"), 40);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/15_easy"), 315);
}

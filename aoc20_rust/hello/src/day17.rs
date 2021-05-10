use std::collections::HashSet;
use std::fs;

pub fn run() {
    let input_file = "data/17";
    let first = part1(input_file, 6);
    println!("{}", first);
    let second = part2(input_file);
    println!("{}", second);
}

fn count_neighbours((x, y, z): (i64, i64, i64), set: &HashSet<(i64, i64, i64)>) -> i64 {
    let mut neighbours = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                if i == 0 && j == 0 && k == 0 {
                    continue;
                }
                if set.contains(&(x + i, y + j, z + k)) {
                    neighbours += 1;
                }
            }
        }
    }
    neighbours
}

fn count_neighbours_4d((x, y, z, w): (i64, i64, i64, i64), set: &HashSet<(i64, i64, i64, i64)>) -> i64 {
    let mut neighbours = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                for l in -1..=1 {
                    if i == 0 && j == 0 && k == 0 && l == 0 {
                        continue;
                    }
                    if set.contains(&(x + i, y + j, z + k, w + l)) {
                        neighbours += 1;
                    }
                }
            }
        }
    }
    neighbours
}

fn update_field(set: HashSet<(i64, i64, i64)>) -> HashSet<(i64, i64, i64)> {
    let mut new_set: HashSet<(i64, i64, i64)> = HashSet::new();

    let min_x = set.iter().map(|(x, _, _)| x).min().unwrap();
    let max_x = set.iter().map(|(x, _, _)| x).max().unwrap();
    let min_y = set.iter().map(|(_, y, _)| y).min().unwrap();
    let max_y = set.iter().map(|(_, y, _)| y).max().unwrap();
    let min_z = set.iter().map(|(_, _, z)| z).min().unwrap();
    let max_z = set.iter().map(|(_, _, z)| z).max().unwrap();

    for x in min_x - 1..=max_x + 1 {
        for y in min_y - 1..=max_y + 1 {
            for z in min_z - 1..=max_z + 1 {
                match count_neighbours((x, y, z), &set) {
                    2 => {
                        if set.contains(&(x, y, z)) {
                            new_set.insert((x, y, z));
                        }
                    }
                    3 => {
                        new_set.insert((x, y, z));
                    }
                    _ => (),
                }
            }
        }
    }
    
    new_set
}

fn update_field_4d(set: HashSet<(i64, i64, i64, i64)>) -> HashSet<(i64, i64, i64, i64)> {
    let mut new_set: HashSet<(i64, i64, i64, i64)> = HashSet::new();

    let min_x = set.iter().map(|(x, _, _, _)| x).min().unwrap();
    let max_x = set.iter().map(|(x, _, _, _)| x).max().unwrap();
    let min_y = set.iter().map(|(_, y, _, _)| y).min().unwrap();
    let max_y = set.iter().map(|(_, y, _, _)| y).max().unwrap();
    let min_z = set.iter().map(|(_, _, z, _)| z).min().unwrap();
    let max_z = set.iter().map(|(_, _, z, _)| z).max().unwrap();
    let min_w = set.iter().map(|(_, _, _, w)| w).min().unwrap();
    let max_w = set.iter().map(|(_, _, _, w)| w).max().unwrap();
    
    for x in min_x - 1..=max_x + 1 {
        for y in min_y - 1..=max_y + 1 {
            for z in min_z - 1..=max_z + 1 {
                for w in min_w - 1..=max_w + 1 {
                    match count_neighbours_4d((x, y, z, w), &set) {
                        2 => {
                            if set.contains(&(x, y, z, w)) {
                                new_set.insert((x, y, z, w));
                            }
                        }
                        3 => {
                            new_set.insert((x, y, z, w));
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    
    new_set
}

fn _print_field(set: &HashSet<(i64, i64, i64)>) {
    let min_x = set.iter().map(|(x, _, _)| x).min().unwrap();
    let max_x = set.iter().map(|(x, _, _)| x).max().unwrap();
    let min_y = set.iter().map(|(_, y, _)| y).min().unwrap();
    let max_y = set.iter().map(|(_, y, _)| y).max().unwrap();
    let min_z = set.iter().map(|(_, _, z)| z).min().unwrap();
    let max_z = set.iter().map(|(_, _, z)| z).max().unwrap();
    println!("===");
    println!();
    for z in min_z - 0..=max_z + 0 {
        for y in min_y - 0..=max_y + 0 {
            for x in min_x - 0..=max_x + 0 {
                if set.contains(&(x, y, z)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
        println!();
    }
}

fn part1(input: &str, iterations: i64) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let mut field_set: HashSet<(i64, i64, i64)> = s
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(j, _)| ((j as i64, i as i64, 0_i64)))
        })
        .collect();
    for _ in 0..iterations {
        field_set = update_field(field_set);
    }
    
    field_set.len() as i64
}

fn part2(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let mut field_set: HashSet<(i64, i64, i64, i64)> = s
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(j, _)| ((j as i64, i as i64, 0_i64, 0_i64)))
        })
        .collect();
    for _ in 0..6 {
        field_set = update_field_4d(field_set);
    }
    
    field_set.len() as i64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_easy() {
        assert_eq!(part1("data/17_easy", 6), 112);
    }

    #[test]
    fn test_part2_easy() {
        assert_eq!(part2("data/17_easy"), 848);
    }
}

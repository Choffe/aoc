use std::fs;

pub fn run() {
    let first = part1("data/13");
    println!("{}", first);
    let second = part2("data/13");
    println!("{}", second);
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let input_parts: Vec<&str> = s.split("\n\n").collect();
    let mut coords: Vec<(i32, i32)> = input_parts[0]
        .lines()
        .map(|l| {
            let mut split = l.split(',');
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let instructions = input_parts[1].lines();
    for line in instructions {
        let mut split = line.split('=');
        let direction = split.next().unwrap().chars().last().unwrap();
        let fold_line: i32 = split.next().unwrap().parse().unwrap();
        coords = match direction {
            'x' => fold_x(coords, fold_line),
            'y' => fold_y(coords, fold_line),
            _ => panic!("Unknown fold direction"),
        };
        break;
    }
    coords.sort();
    coords.dedup();
    coords.len() as i32
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let input_parts: Vec<&str> = s.split("\n\n").collect();
    let mut coords: Vec<(i32, i32)> = input_parts[0]
        .lines()
        .map(|l| {
            let mut split = l.split(',');
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let instructions = input_parts[1].lines();
    for line in instructions {
        let mut split = line.split('=');
        let direction = split.next().unwrap().chars().last().unwrap();
        let fold_line: i32 = split.next().unwrap().parse().unwrap();
        coords = match direction {
            'x' => fold_x(coords, fold_line),
            'y' => fold_y(coords, fold_line),
            _ => panic!("Unknown fold direction"),
        };
    }
    for y in 0..8 {
        for x in 0..40 {
            if coords.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    0
}

fn fold_x(coords: Vec<(i32, i32)>, fold_line: i32) -> Vec<(i32, i32)> {
    let mut folded_coords = vec![];
    for (x, y) in coords {
        if x < fold_line {
            folded_coords.push((x, y));
        } else {
            let folded_x = fold_line - (x - fold_line);
            if !folded_coords.contains(&(folded_x, y)) {
                folded_coords.push((folded_x, y));
            }
        }
    }
    folded_coords
}

fn fold_y(coords: Vec<(i32, i32)>, fold_line: i32) -> Vec<(i32, i32)> {
    let mut folded_coords = vec![];
    for (x, y) in coords {
        if y < fold_line {
            folded_coords.push((x, y));
        } else {
            let folded_y = fold_line - (y - fold_line);
            if !folded_coords.contains(&(x, folded_y)) {
                folded_coords.push((x, folded_y));
            }
        }
    }
    folded_coords
}
#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/13_easy"), 17);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/13_easy"), 0);
}

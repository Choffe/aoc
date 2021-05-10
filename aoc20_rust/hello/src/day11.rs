use std::convert::TryInto;
use std::fs;

pub fn run() {
    let first = part1("data/11");
    println!("{}", first);
    let second = part2("data/11");
    println!("{}", second);
}

fn update_index(
    field: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    row_lenght: usize,
    col_lenght: usize,
) -> char {
    if field[row][col] == '.' {
        return '.';
    }
    let indices: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let occupied = indices.iter().map(|(i, j)| {
        if i + row as i32 >= 0
            && i + (row as i32) < row_lenght as i32
            && j + col as i32 >= 0
            && j + (col as i32) < col_lenght as i32
        {   
            let r: usize = (*i + row as i32).try_into().unwrap();
            let c: usize = (*j + col as i32).try_into().unwrap();
            match field[r][c] {
                '#' => 1,
                _ => 0,
            }
        } else {
            0
        }
    }).filter(|&x| x == 1).count();
    
    match field[row][col] {
        '#' => {
            if occupied >= 4 {
                return 'L';
            } else {
                return '#';
            }
        }
        'L' => {
            if occupied == 0 {
                return '#';
            } else {
                return 'L';
            }
        }
        _ => panic!("This should not happend"),
    };
}

fn update_index_line_of_sight(
    field: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    row_lenght: usize,
    col_lenght: usize,
) -> char {
    if field[row][col] == '.' {
        return '.';
    }
    let indices: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let mut occupied = 0;
    for (i, j) in indices.iter() {
        let mut first_seen = '.';
        let mut step = 1;
        while first_seen == '.' {
            if (step * i) + row as i32 >= 0
                && (step * i) + (row as i32) < row_lenght as i32
                && (step * j) + col as i32 >= 0
                && (step * j) + (col as i32) < col_lenght as i32
            {   
                let r: usize = (step * (*i) + row as i32).try_into().unwrap();
                let c: usize = (step * (*j) + col as i32).try_into().unwrap();
                first_seen = field[r][c];
            } else {
                first_seen = 'W';
            }
            step += 1;
        } 
        match first_seen {
            '#' => occupied += 1,
            _ => (),
        };
    }
    match field[row][col] {
        '#' => {
            if occupied >= 5 {
                return 'L';
            } else {
                return '#';
            }
        }
        'L' => {
            if occupied == 0 {
                return '#';
            } else {
                return 'L';
            }
        }
        _ => return '.',//panic!("This should not happend, {} {} {}", field[row][col], row, col),
    };
}

fn update_field(field: &mut Vec<Vec<char>>, line_of_sight: bool) {
    let old_field = field.clone();
    // field.iter().enumerate().map(|(row, value)| value.iter().enumerate().map(|(col, _)| update_index(&old_field, row, col, field.len(), field[0].len())));
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if !line_of_sight {
                field[i][j] = update_index(&old_field, i, j, field.len(), field[0].len());
            } else {
                field[i][j] = update_index_line_of_sight(&old_field, i, j, field.len(), field[0].len());
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let s = fs::read_to_string(input).unwrap();
    let mut field: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
    let mut last_count = 0;
    loop {
        update_field(&mut field, false);
        let count: usize = field
            .iter()
            .map(|row| row.iter().filter(|&x| *x == '#').count())
            .sum();
        if last_count == count {
            return count;
        }
        last_count = count;
    }
}

fn part2(input: &str) -> usize {
    let s = fs::read_to_string(input).unwrap();
    let mut field: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
    let mut last_count = 0;
    loop {
        update_field(&mut field, true);
        let count: usize = field
            .iter()
            .map(|row| row.iter().filter(|&x| *x == '#').count())
            .sum();
        if last_count == count {
            return count;
        }
        last_count = count;
    }
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/11_easy"), 37);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/11_easy"), 26);
}

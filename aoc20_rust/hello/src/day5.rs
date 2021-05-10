use std::fs;

pub fn run() {
    let first = part1("data/5");
    println!("{}", first);
    let second = part2("data/5");
    println!("{}", second);
}

fn part1(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = s.split("\n").collect();
    let mut max_seat_id = -1;
    for line in lines {
        let (row, col) = line.split_at(7);
        let row = row.replace('F', "0").replace('B', "1");
        let row = i64::from_str_radix(&row, 2).expect("Can't parse as binary");
        let col = col.replace('L', "0").replace('R', "1");
        let col = i64::from_str_radix(&col, 2).expect("Can't parse as binary");
        let seat_id = row * 8 + col;
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }
    max_seat_id
}

fn part2(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let lines = s.split("\n");
    let mut sorted_seat_id = lines.map(|line| {
        let (row, col) = line.split_at(7);
        let row = row.replace('F', "0").replace('B', "1");
        let row = i64::from_str_radix(&row, 2).expect("Can't parse as binary");
        let col = col.replace('L', "0").replace('R', "1");
        let col = i64::from_str_radix(&col, 2).expect("Can't parse as binary");
        let seat_id = row * 8 + col;
        seat_id
    }).collect::<Vec<i64>>();
    sorted_seat_id.sort();
    for i in 0..sorted_seat_id.len()-1 {
        if sorted_seat_id[i] + 2 == sorted_seat_id[i+1] {
            return sorted_seat_id[i] + 1;
        }
    }
    -1
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/5_easy"), 820);
}

// #[test]
// fn test_part2_easy() {
//     assert_eq!(part2("data/5_easy"), 0);
// }

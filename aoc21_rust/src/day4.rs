use std::fs;

pub fn run() {
    let first = part1("data/4");
    println!("{}", first);
    let second = part2("data/4");
    println!("{}", second);
}

#[derive(Debug, PartialEq)]
struct Board {
    combinations: Vec<BingoRow>,
}
// we dont add columns and when we calc score we calc them some twice right? so devide by 2
impl Board {
    fn new(lines: &Vec<String>, i: &usize) -> Board {
        let index = *i;
        let board_size = 5;
        let mut board = Board {
            combinations: vec![],
        };
        let mut columns = vec![
            BingoRow {
                row: vec![],
                striked: vec![false; board_size]
            };
            5
        ];
        for i in index..index + board_size {
            let row: Vec<i32> = lines[i]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            board.add(BingoRow {
                row: row.clone(),
                striked: vec![false; board_size],
            });
            for j in 0..5 {
                columns[j].row.push(row[j]);
            }
        }

        for column in columns {
            board.add(column);
        }

        board
    }

    fn add(&mut self, row: BingoRow) {
        self.combinations.push(row);
    }

    fn register(&mut self, value: &i32) {
        for br in &mut self.combinations {
            br.register(value);
        }
    }

    fn won(&self) -> bool {
        self.combinations.iter().any(|br| br.won())
    }

    fn score(&self) -> i32 {
        self.combinations.iter().fold(0, |acc, br| acc + br.score()) / 2
    }
}
#[derive(Debug, Clone, PartialEq)]
struct BingoRow {
    row: Vec<i32>,
    striked: Vec<bool>,
}

impl BingoRow {
    fn register(&mut self, value: &i32) {
        if let Some(i) = self.row.iter().position(|r| r == value) {
            self.striked[i] = true;
        }
    }

    fn won(&self) -> bool {
        self.striked.iter().all(|x| *x)
    }

    fn score(&self) -> i32 {
        let mut sum = 0;
        for i in 0..self.row.len() {
            if !self.striked[i] {
                sum += self.row[i];
            }
        }
        sum
    }
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|x| String::from(x))
        .collect::<Vec<String>>();
    let values: Vec<i32> = s[0].split(',').map(|x| x.parse().unwrap()).collect();

    let mut boards = vec![];
    let mut i = 2;
    while i < s.len() {
        boards.push(Board::new(&s, &i));
        i += 6;
    }
    for v in values {
        for b in &mut boards {
            b.register(&v);
        }
        let winners: Vec<&Board> = boards.iter().filter(|b| b.won()).collect();
        if winners.len() > 0 {
            return winners[0].score() * v;
        }
    }

    0
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|x| String::from(x))
        .collect::<Vec<String>>();
    let values: Vec<i32> = s[0].split(',').map(|x| x.parse().unwrap()).collect();

    let mut boards = vec![];
    let mut i = 2;
    while i < s.len() {
        boards.push(Board::new(&s, &i));
        i += 6;
    }
    for v in values {
        for b in &mut boards {
            b.register(&v);
        }
        if boards.len() == 1 && boards[0].won() {
            return boards[0].score() * v;
        }
        boards = boards.into_iter().filter(|b| !b.won()).collect();
    }

    0
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/4_easy"), 4512);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/4_easy"), 1924);
}

use regex::Regex;
use std::fs;

struct Passport<'a> {
    data: &'a str,
}
//guessed 161 -> too high
//guessed 134 -> too low

impl Passport<'_> {
    fn check_iyr(data: &str) -> bool {
        let pattern = Regex::new(r"iyr:(\d{4})\b").unwrap();
        let year: i32 = match pattern.captures(data) {
            Some(cap) => cap.get(1),
            None => return false,
        }.unwrap().as_str().parse().unwrap();
        if year >= 2010 && year <= 2020 {
            true
        } else {
            false
        }
    }

    fn check_byr(data: &str) -> bool {
        let pattern = Regex::new(r"byr:(\d{4})\b").unwrap();
        let year: i32 = match pattern.captures(data) {
            Some(cap) => cap.get(1),
            None => return false,
        }.unwrap().as_str().parse().unwrap();
        if year >= 1920 && year <= 2002 {
            true
        } else {
            false
        }
    }

    fn check_eyr(data: &str) -> bool {
        let pattern = Regex::new(r"eyr:(\d{4})\b").unwrap();
        let year: i32 = match pattern.captures(data) {
            Some(cap) => cap.get(1),
            None => return false,
        }.unwrap().as_str().parse().unwrap();
        if year >= 2020 && year <= 2030 {
            true
        } else {
            false
        }
    }

    fn check_pid(data: &str) -> bool {
        let pattern = Regex::new(r"pid:(\d{9})\b").unwrap();
        pattern.is_match(data)
    }

    fn check_hcl(data: &str) -> bool {
        let pattern = Regex::new(r"hcl:#([0123456789abcdef]{6}\b)").unwrap();
        pattern.is_match(data)
    }

    fn check_hgt(data: &str) -> bool {
        let pattern = Regex::new(r"hgt:(\d+)(\w+)\b").unwrap();
        let (length, unit) = match pattern.captures(data) {
            Some(cap) => (cap.get(1).unwrap().as_str().parse::<i32>().unwrap(), cap.get(2).unwrap().as_str()),
            None => return false,
        };
        if unit == "in" && length >= 59 && length <= 76 {
            true
        } else if unit == "cm" && length >= 150 && length <= 193 {
            true
        } else {
            false
        }
    }

    fn check_ecl(data: &str) -> bool {
        let pattern = Regex::new(r"ecl:(\w+)\b").unwrap();
        let ecl = match pattern.captures(data) {
            Some(cap) => cap.get(1).unwrap().as_str(),
            None => return false,
        };
        if ecl == "amb" || ecl == "blu" || ecl == "brn" || ecl == "gry" || ecl == "grn" || ecl == "hzl" || ecl == "oth" {
            true
        } else {
            false
        }
    }

    fn is_valid(&self) -> bool {
        Passport::check_iyr(self.data)
        && Passport::check_ecl(self.data)
        && Passport::check_eyr(self.data)
        && Passport::check_pid(self.data)
        && Passport::check_hcl(self.data)
        && Passport::check_hgt(self.data)
        && Passport::check_byr(self.data)
    }
}

pub fn run() {
    let first = part1("data/4");
    println!("{}", first);
    let second = part2("data/4");
    println!("{}", second);
}

fn part1(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = s.split("\n\n").collect();
    let checker = vec![
        Regex::new(r"iyr:").unwrap(),
        Regex::new(r"ecl:").unwrap(),
        Regex::new(r"eyr:").unwrap(),
        Regex::new(r"pid:").unwrap(),
        Regex::new(r"hcl:").unwrap(),
        Regex::new(r"hgt:").unwrap(),
        Regex::new(r"byr:").unwrap(),
    ];
    let nbr_valid = lines
        .iter()
        .map(|line| checker.iter().map(|x| x.is_match(line)).all(|x| x))
        .fold(0, |acc, x| if x { acc + 1 } else { acc });
    nbr_valid
}

fn part2(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = s.split("\n\n").collect();
    let nbr_valid = lines
        .iter()
        .map(|line| {
            (Passport { data: line }).is_valid()
        })
        .fold(0, |acc, x| if x { acc + 1 } else { acc });
    nbr_valid
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/4_easy"), 2);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/4_medium"), 4);
}

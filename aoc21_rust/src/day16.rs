use std::fs;

pub fn run() {
    let first = part1("data/16");
    println!("{}", first);
    let second = part2("data/16");
    println!("{}", second);
}

fn part1(input: &str) -> i128 {
    let s = fs::read_to_string(input).unwrap();
    let s = get_binary(&s);
    let packet = Packet::create(&s, 0);
    packet.get_version()
}

fn part2(input: &str) -> i128 {
    let s = fs::read_to_string(input).unwrap();
    let s = get_binary(&s);
    let packet = Packet::create(&s, 0);
    packet.get_value()
}

fn get_binary(input: &str) -> String {
    let value: Vec<_> = input
        .chars()
        .filter_map(|c| i128::from_str_radix(&c.to_string(), 16).ok())
        .map(|d| format!("{:04b}", d))
        .collect();
    value.join("")
}

enum Packet {
    Lit(Litteral),
    Op(Operator),
}

impl Packet {
    fn create(s: &str, index: usize) -> Packet {
        let mut version_sum = 0;
        let mut i = index;
        let version = i128::from_str_radix(s.get(i..i + 3).unwrap(), 2).unwrap();
        i += 3;
        let type_id = i128::from_str_radix(s.get(i..i + 3).unwrap(), 2).unwrap();
        i += 3;
        let mut value: Vec<_> = vec![];
        if type_id == 4 {
            loop {
                let keep_on = i128::from_str_radix(s.get(i..i + 1).unwrap(), 2).unwrap();
                i += 1;
                value.push(s.get(i..i + 4).unwrap());
                i += 4;
                if keep_on == 0 {
                    break;
                }
            }
            let value = i128::from_str_radix(&value.join(""), 2).unwrap();
            let l = Litteral {
                version: version,
                value: value,
                length: i - index,
            };
            return Packet::Lit(l);
        } else {
            let mut sub_packages = vec![];
            let length_type_id = i128::from_str_radix(s.get(i..i + 1).unwrap(), 2).unwrap();
            i += 1;
            if length_type_id == 0 {
                let length_in_bits = i128::from_str_radix(s.get(i..i + 15).unwrap(), 2).unwrap();
                i += 15;
                let mut read_bits = 0;
                while read_bits < length_in_bits {
                    let p = Packet::create(s, i);
                    read_bits += p.get_length() as i128;
                    i += p.get_length();
                    version_sum += p.get_version();
                    sub_packages.push(p);
                }
            } else {
                let nbr_sub_package = i128::from_str_radix(s.get(i..i + 11).unwrap(), 2).unwrap();
                i += 11;
                let mut created_nbr = 0;
                while created_nbr < nbr_sub_package {
                    let p = Packet::create(s, i);
                    created_nbr += 1;
                    i += p.get_length();
                    version_sum += p.get_version();
                    sub_packages.push(p);
                }
            }
            let mut o = Operator {
                version: version + version_sum,
                value: 0,
                length: i - index,
            };
            match type_id {
                0 => {
                    o.value = sub_packages.iter().map(|p| p.get_value()).sum();
                    ()
                }
                1 => {
                    o.value = sub_packages.iter().map(|p| p.get_value()).product();
                    ()
                }
                2 => {
                    o.value = sub_packages.iter().map(|p| p.get_value()).min().unwrap();
                    ()
                }
                3 => {
                    o.value = sub_packages.iter().map(|p| p.get_value()).max().unwrap();
                    ()
                }
                5 => {
                    if sub_packages[0].get_value() > sub_packages[1].get_value() {
                        o.value = 1;
                    } else {
                        o.value = 0;
                    }
                }
                6 => {
                    if sub_packages[0].get_value() < sub_packages[1].get_value() {
                        o.value = 1;
                    } else {
                        o.value = 0;
                    }
                }
                7 => {
                    if sub_packages[0].get_value() == sub_packages[1].get_value() {
                        o.value = 1;
                    } else {
                        o.value = 0;
                    }
                }
                _ => {
                    panic!("Unsupported type id");
                }
            };
            return Packet::Op(o);
        }
    }

    fn get_length(&self) -> usize {
        match self {
            Packet::Lit(x) => x.length,
            Packet::Op(x) => x.length,
        }
    }

    fn get_value(&self) -> i128 {
        match self {
            Packet::Lit(x) => x.value,
            Packet::Op(x) => x.value,
        }
    }

    fn get_version(&self) -> i128 {
        match self {
            Packet::Lit(x) => x.version,
            Packet::Op(x) => x.version,
        }
    }
}

struct Operator {
    version: i128,
    value: i128,
    length: usize,
}

struct Litteral {
    version: i128,
    value: i128,
    length: usize,
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/16_easy"), 981);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/16_easy"), 299227024091);
}

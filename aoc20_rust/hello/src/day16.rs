use std::fs;
use std::collections::HashSet;

pub fn run() {
    let input_file = "data/16";
    let first = part1(input_file);
    println!("{}", first);
    let second = part2(input_file);
    println!("{}", second);
}

#[derive(Debug, Clone)]
struct Ticket {
    values: Vec<i64>,
    invalid_values: Vec<i64>,
    valid_rules: Vec<HashSet<String>>,
}

#[derive(Debug)]
struct Rule {
    name: String,
    valid_ranges: Vec<(i64, i64)>,
}

impl Rule {
    fn new(rule_str: &str) -> Rule {
        let mut valid_ranges = vec![];
        let mut splitter = rule_str.splitn(2, ":");
        let name = splitter.next();
        let splitter = splitter.next().unwrap().trim().split("or");
        for r in splitter {
            let ranges: Vec<i64> = r
                .split("-")
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect();
            valid_ranges.push((ranges[0], ranges[1]));
        }
        Rule {
            name: String::from(name.unwrap()),
            valid_ranges: valid_ranges,
        }
    }

    fn valid(&self, value: &i64) -> bool {
        self.valid_ranges
            .iter()
            .any(|(first, last)| value >= first && value <= last)
    }
}

impl Ticket {
    fn new(input: &str, rules: &Vec<Rule>) -> Ticket {
        let mut ticket = Ticket {
            values: input.split(",").map(|x| x.parse().unwrap()).collect(),
            invalid_values: vec![],
            valid_rules: vec![HashSet::new(); 20],
        };
        ticket.validate(rules);
        ticket
    }
    fn validate(&mut self, rules: &Vec<Rule>) {
        for (i, value) in self.values.iter().enumerate() {
            if rules.iter().all(|x| !x.valid(value)) {
                self.invalid_values.push(*value);
            }
            for rule in rules {
                if rule.valid(value) {
                    self.valid_rules[i].insert(String::from(&rule.name));
                }
            }
        }
    }
}

fn part1(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let ticket: Vec<&str> = s.split("\r\n\r\n").collect();
    if ticket.len() != 3 {
        println!("{:?}", ticket);
        return -1;
    }
    let rules = ticket[0];
    let rules: Vec<Rule> = rules.lines().map(|x| Rule::new(x)).collect();
    let neighbours_tickets = ticket[2];
    neighbours_tickets
        .lines()
        .skip(1)
        .map(|x| Ticket::new(x, &rules))
        .filter(|x| x.invalid_values.len() > 0)
        .fold(0, |acc: i64, x| acc + x.invalid_values.iter().sum::<i64>())
}

fn part2(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let ticket: Vec<&str> = s.split("\r\n\r\n").collect();
    if ticket.len() != 3 {
        panic!("Must have lenght 3");
    }
    let rules = ticket[0];
    let my_ticket = ticket[1].lines().nth(1).unwrap();
    let rules: Vec<Rule> = rules.lines().map(|x| Rule::new(x)).collect();
    let my_ticket = Ticket::new(my_ticket, &rules);
    let neighbours_tickets = ticket[2];
    let mut valid_neighbours: Vec<Ticket> = neighbours_tickets
        .lines()
        .skip(1)
        .map(|x| Ticket::new(x, &rules))
        .filter(|x| x.invalid_values.len() == 0)
        .collect();
    valid_neighbours.push(my_ticket.clone());
    
    let mut possible_names_for_field: Vec<Vec<_>> = vec![vec![]; 20];
    for i in 0..20 {
        let mut vec: Vec<_> = vec![];
        for r in valid_neighbours[0].valid_rules[i].iter() {
            vec.push(String::from(r));
        }
        for t in 1..valid_neighbours.len() {
            let new_set: HashSet<String> = vec.into_iter().collect();
            vec = new_set.intersection(&valid_neighbours[t].valid_rules[i]).cloned().collect();
        }
        for rule_name in vec {
            possible_names_for_field[i].push(rule_name);
        }
    }
    
    let mut names_for_field: Vec<String> = vec![String::from(""); 20];
    for _ in 0..20 {
        let single_entry = possible_names_for_field.iter().enumerate().filter(|(_,x)| x.len() == 1).collect::<Vec<_>>();
        if single_entry.len() < 1 {
            continue;
        }
        let single_entry = single_entry[0];
        let index = single_entry.0;
        let single_name = single_entry.1[0].clone();
        for j in 0..possible_names_for_field.len() {
            possible_names_for_field[j].retain(|x| x != &single_name);
        }
        names_for_field[index] = single_name;
    }
    let result: i64 = names_for_field.iter().enumerate().filter(|(_, name)| name.contains("departure")).map(|(index, _)| my_ticket.values[index]).product();
    
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rule() {
        let rule = Rule {
            name: String::from("test"),
            valid_ranges: vec![(1, 3), (5, 7)],
        };
        let value = 7;
        assert!(rule.valid(&value));
    }

    #[test]
    fn test_part1_easy() {
        assert_eq!(part1("data/16_easy"), 71);
    }
    #[test]
    fn test_part2_easy() {
        //not valid since it doesnt contain depature
        assert_eq!(part2("data/16_easy"), 1);
    }
}

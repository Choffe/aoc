use std::fs;

pub fn run() {
    let first = part1("data/12");
    println!("{}", first);
    let second = part2("data/12");
    println!("{}", second);
}
//part2 guessed 58041 answer is to low
#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Ship {
    position: (i32, i32),
    direction: Direction,
    waypoint: (i32, i32)
}

impl Ship {
    fn execute(&mut self, instructrion: &str) {
        let (op, value) = instructrion.split_at(1);
        match op {
            "F" => self.forward(value.parse::<i32>().unwrap()),
            "R" | "L" => self.turn(op, value.parse::<i32>().unwrap()),
            "N" | "S" | "E" | "W" => self.drift(op, value.parse::<i32>().unwrap()),
            _ => panic!("{} Unknown operation", op),
        };
    }

    fn execute_waypoint(&mut self, instructrion: &str) {
        let (op, value) = instructrion.split_at(1);
        match op {
            "F" => self.forward_waypoint(value.parse::<i32>().unwrap()),
            "R" | "L" => self.turn_waypoint(op, value.parse::<i32>().unwrap()),
            "N" | "S" | "E" | "W" => self.drift_waypoint(op, value.parse::<i32>().unwrap()),
            _ => panic!("{} Unknown operation", op),
        };
    }

    fn forward_waypoint(&mut self, value: i32) {
        self.position.0 += self.waypoint.0 * value;
        self.position.1 += self.waypoint.1 * value;
    }
    
    fn drift_waypoint(&mut self, dir: &str, value: i32) {
        match dir {
            "N" => self.waypoint.1 += value,
            "S" => self.waypoint.1 -= value,
            "E" => self.waypoint.0 += value,
            "W" => self.waypoint.0 -= value,
            _ => panic!("{} Drift waypoint with invalid direction"),
        };
    }

    fn turn_waypoint(&mut self, dir: &str, value: i32) {
        let mut steps = value / 90 % 4;
        if (steps == 1 || steps == 3) && dir == "L" {
            steps = (steps + 2) % 4;
        }
        let (x, y) = self.waypoint;
        match steps {
            0 => (),
            1 => {
                self.waypoint.0 = y;
                self.waypoint.1 = -x;
            },
            2 => {
                self.waypoint.0 = -x;
                self.waypoint.1 = -y;
            },
            3 => {
                self.waypoint.0 = -y;
                self.waypoint.1 = x;
            },
            _ => panic!("{} Steps should only be 0 - 3", steps)
        };
    }

    fn drift(&mut self, dir: &str, value: i32) {
        match dir {
            "N" => self.position.1 += value,
            "S" => self.position.1 -= value,
            "E" => self.position.0 += value,
            "W" => self.position.0 -= value,
            _ => panic!("{} Drift with invalid direction"),
        };
    }

    fn turn(&mut self, dir: &str, value: i32) {
        let mut directions;
        if dir == "R" {
            directions = [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .iter()
            .cycle();
        } else {
            directions = [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ]
            .iter()
            .cycle();
        }
        while directions.next() != Some(&self.direction) {}
        let steps = (value / 90) % 4;
        for _ in 0..steps {
            self.direction = *directions.next().unwrap();
        }
    }

    fn forward(&mut self, value: i32) {
        match self.direction {
            Direction::North => self.position.1 += value,
            Direction::South => self.position.1 -= value,
            Direction::East => self.position.0 += value,
            Direction::West => self.position.0 -= value,
        };
    }

    fn manhattan_dist(&self) -> i32 {
        let (x, y) = self.position;
        i32::abs(x) + i32::abs(y)
    }
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let mut ship = Ship {
        position: (0, 0),
        direction: Direction::East,
        waypoint: (10, 1)
    };
    for instruction in s.lines() {
        ship.execute(instruction);
    }
    ship.manhattan_dist()
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let mut ship = Ship {
        position: (0, 0),
        direction: Direction::East,
        waypoint: (10, 1)
    };
    for instruction in s.lines() {
        ship.execute_waypoint(instruction);
    }
    ship.manhattan_dist()
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/12_easy"), 25);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/12_easy"), 286);
}

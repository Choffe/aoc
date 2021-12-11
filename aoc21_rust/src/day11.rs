use std::fs;

pub fn run() {
    let first = part1("data/11");
    println!("{}", first);
    let second = part2("data/11");
    println!("{}", second);
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let mut g = Grid::new(s.chars().filter_map(|c| c.to_digit(10)).collect());

    for _ in 0..100 {
        g.increase();
        g.flash();
        g.reset();
    }
    g.flashes()
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let mut g = Grid::new(s.chars().filter_map(|c| c.to_digit(10)).collect());

    for i in 1.. {
        g.increase();
        g.flash();
        if g.step_flashes() as usize == g.grid.len() * g.grid.len() {
            return i;
        }
        g.reset();
    }
    -1
}

struct Grid {
    grid: [[Octupus; 10]; 10],
    flashes: i32,
}

#[derive(Copy, Clone)]
struct Octupus {
    value: i32,
    flashed: bool,
}

impl Grid {
    fn new(input: Vec<u32>) -> Grid {
        let mut g = Grid {
            grid: [[Octupus {
                value: 0,
                flashed: false,
            }; 10]; 10],
            flashes: 0,
        };
        for y in 0..g.grid.len() {
            for x in 0..g.grid.len() {
                g.grid[y][x].value = input[y * 10 + x] as i32;
            }
        }

        g
    }
    fn _print(&self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid.len() {
                print!("{}", self.grid[y][x].value);
            }
            println!();
        }
        println!();
    }

    fn increase(&mut self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid.len() {
                self.grid[y][x].value += 1;
            }
        }
    }

    fn is_valid(&self, coord: &(i32, i32)) -> bool {
        let (x, y) = *coord;
        x >= 0 && x < self.grid.len() as i32 && y >= 0 && y < self.grid.len() as i32
    }

    fn flash_coord(&mut self, x: usize, y: usize) {
        self.grid[y][x].flashed = true;
        let neighbours: [(i32, i32); 8] = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];
        let neighbours: Vec<_> = neighbours
            .iter()
            .map(|(nx, ny)| (x as i32 + nx, y as i32 + ny))
            .filter(|c| self.is_valid(c))
            .collect();

        for (gx, gy) in neighbours {
            self.grid[gy as usize][gx as usize].value += 1;
            if self.grid[gy as usize][gx as usize].value >= 10
                && !self.grid[gy as usize][gx as usize].flashed
            {
                self.flash_coord(gx as usize, gy as usize);
            }
        }
    }

    fn flash(&mut self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid.len() {
                if self.grid[y][x].value >= 10 && !self.grid[y][x].flashed {
                    self.flash_coord(x, y);
                }
            }
        }
    }

    fn reset(&mut self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid.len() {
                if self.grid[y][x].value > 9 {
                    assert_eq!(self.grid[y][x].flashed, true);
                    self.grid[y][x].value = 0;
                    self.grid[y][x].flashed = false;
                    self.flashes += 1;
                }
            }
        }
    }

    fn flashes(&self) -> i32 {
        self.flashes
    }

    fn step_flashes(&self) -> i32 {
        let mut step_flashes = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid.len() {
                if self.grid[y][x].flashed {
                    step_flashes += 1;
                }
            }
        }
        step_flashes
    }
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/11_easy"), 1656);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/11_easy"), 195);
}

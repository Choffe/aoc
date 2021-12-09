use std::fs;

pub fn run() {
    let first = part1("data/9");
    println!("{}", first);
    let second = part2("data/9");
    println!("{}", second);
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(map: Vec<Vec<u32>>) -> Grid {
        let height = map.len();
        let width = map[0].len();
        Grid {
            grid: map,
            height: height,
            width: width,
        }
    }

    fn is_valid(&self, point: &(i32, i32)) -> bool {
        let (x, y) = *point;
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    fn get_neighbours(&self, x: &usize, y: &usize) -> Vec<(usize, usize)> {
        let mut neighbours: Vec<(i32, i32)> = vec![];
        neighbours.push((x.clone() as i32 - 1, y.clone() as i32));
        neighbours.push((x.clone() as i32, y.clone() as i32 - 1));
        neighbours.push((x.clone() as i32, y.clone() as i32 + 1));
        neighbours.push((x.clone() as i32 + 1, y.clone() as i32));
        neighbours
            .into_iter()
            .filter(|p| self.is_valid(&p))
            .map(|(x, y)| (x as usize, y as usize))
            .collect()
    }

    fn is_lowpoint(&self, x: &usize, y: &usize) -> bool {
        let neighbours = self.get_neighbours(x, y);
        let value = self.grid[*y][*x];

        neighbours
            .iter()
            .map(|(x, y)| self.grid[*y][*x])
            .all(|v| v > value)
    }

    fn calc_basin_size(&self, visited: &mut Vec<(usize, usize)>, x: usize, y: usize) -> i32 {
        if visited.contains(&(x, y)) {
            return 0;
        }
        visited.push((x, y));

        if self.grid[y][x] == 9 {
            return 0;
        }

        self.get_neighbours(&x, &y)
            .iter()
            .map(|(nx, ny)| self.calc_basin_size(&mut *visited, *nx, *ny))
            .sum::<i32>()
            + 1
    }
}

fn part1(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<Vec<u32>> = s
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    let grid = Grid::new(lines);

    let mut risk = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.is_lowpoint(&x, &y) {
                risk += 1 + grid.grid[y][x];
            }
        }
    }
    risk as i32
}

fn part2(input: &str) -> i32 {
    let s = fs::read_to_string(input).unwrap();
    let lines: Vec<Vec<u32>> = s
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    let grid = Grid::new(lines);

    let mut visited = vec![];
    let mut basins = vec![];
    for y in 0..grid.height {
        for x in 0..grid.width {
            basins.push(grid.calc_basin_size(&mut visited, x, y));
        }
    }
    basins.sort();
    basins.reverse();
    let mut sum = 1;
    for s in 0..3 {
        sum *= basins[s];
    }
    sum
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/9_easy"), 15);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/9_easy"), 1134);
}

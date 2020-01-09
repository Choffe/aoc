use std::fs::read_to_string;

fn main() {
    let content = read_to_string("input/day1.txt").unwrap();

    part_one(&content);
    part_two(&content);
}

fn part_one(content: &String) {
    println!("Part one:");
    let up = content.matches("(").count();
    let down = content.matches(")").count();
    let total = (up as i32) - (down as i32);

    println!("Up number of floors {}", up);
    println!("Down number of floors {}", down);
    println!("Ends up on floor number {}", total);
}

fn part_two(content: &String) {
    println!("\nPart Two:");
    let mut current_floor = 0;
    for (i, c) in content.chars().enumerate() {
        match c {
            '(' => current_floor+=1,
            ')' => current_floor-=1,
            _ => println!("Found char {}", c),
        }
        if current_floor == -1 {
            println!("Found basement in {}", i + 1);
            break;
        }

    }
}
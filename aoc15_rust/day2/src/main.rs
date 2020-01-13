use std::fs::read_to_string;

fn main() {
    println!("Day two:");
    let content = read_to_string("src/input_day2.txt").unwrap();

    part_one(&content);
    part_two(&content);
}

fn part_one(content: &String) {
    let mut paper_needed = 0;
    for s in content.lines() {
        let mut iter = s.split('x');

        let l = iter.next().unwrap().parse::<i32>().unwrap();
        let w = iter.next().unwrap().parse::<i32>().unwrap();
        let h = iter.next().unwrap().parse::<i32>().unwrap();
        let sides = [l*w, w*h, h*l];
        let min_side = sides.iter().min().unwrap();
        let surface_area:Vec<i32> = sides.into_iter().map(|x| x * 2).collect::<Vec<_>>();
        let surface_area:i32 = surface_area.iter().sum();

        paper_needed += surface_area + min_side;
    }
    println!("Wrapping paper needed : {}", paper_needed);

}

fn part_two(content: &String) {
    let mut ribbon_needed = 0;
    for s in content.lines() {
        let mut iter = s.split('x');

        let l = iter.next().unwrap().parse::<i32>().unwrap();
        let w = iter.next().unwrap().parse::<i32>().unwrap();
        let h = iter.next().unwrap().parse::<i32>().unwrap();
        let bow = l * w * h;
        let wrap: i32;
        if h >= w && h >= l {
            wrap = 2 * l + 2 * w;
        } else if w >= h && w >= l {
            wrap = 2 * l + 2 * h;
        } else {
            wrap = 2 * w + 2 * h;
        }
        ribbon_needed += bow + wrap;
    }
    println!("Ribbon needed : {}", ribbon_needed);
}
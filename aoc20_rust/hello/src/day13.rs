use std::fs;

pub fn run() {
    let first = part1("data/13");
    println!("{}", first);
    let second = part2("data/13");
    println!("{}", second);
}

fn part1(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let mut line = s.lines();
    let time:i64 = line.next().unwrap().parse().unwrap();
    let busses: Vec<i64> = line.next().unwrap().split(',').filter(|x| x.parse::<i64>().is_ok()).map(|x| x.parse::<i64>().unwrap()).collect();
    for index in time.. {
        let dep = busses.iter().filter(|&x| index %x == 0).collect::<Vec<&i64>>();
        if dep.len() != 0 {
            return dep[0] * (index - time);
        }
    }

    -1
}

fn part2(input: &str) -> i64 {
    let s = fs::read_to_string(input).unwrap();
    let mut line = s.lines();
    line.next();
    let busses: Vec<i64> = line.next().unwrap().split(',').map(|x| match x.parse::<i64>() {
        Ok(x) => x,
        Err(_) => -1
    }).collect();
    let mut dep = busses.iter().enumerate().filter(|(_, &x)| x > 0).collect::<Vec<(usize, &i64)>>();
    let mut index = 0;
    let mut step = 1;// *min_id.1;
    dep.retain(|(_, &x)| x != step);

    loop {
        let mut i = 0;
        while i < dep.len() {
            if (index + dep[i].0) % *dep[i].1 as usize == 0 {
                let new_id = *dep[i].1;
                step = step * new_id;
                dep.retain(|(_, &x)| x != new_id);
                if i > 0 {
                    i -= 1;
                }
            }
            i += 1;
        }
        if dep.len() == 0 {
            return index as i64;
        } 
        
        index += step as usize;
     
    }
}

#[test]
fn test_part1_easy() {
    assert_eq!(part1("data/13_easy"), 295);
}

#[test]
fn test_part2_easy() {
    assert_eq!(part2("data/13_easy"), 1068781);
}
#[test]
fn test_part2_medium() {
    assert_eq!(part2("data/13_medium"), 754018);
}
//845429781570686 too low
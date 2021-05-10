pub fn run() {
    part1();
    part2();
}

fn part1(){
    let start = 256310;
    let end = 732736;

    let it = start..=end;
    let passwords= it.filter(|x| is_valid_p1(*x)).count();
    println!("{}", passwords);
}

fn part2() {
    let start = 256310;
    let end = 732736;

    let it = start..=end;
    let passwords= it.filter(|x| is_valid_p2(*x)).count();
    println!("{}", passwords);
}

fn nth(index: i32, value: &i32) -> i32 {
    match index {
        0 => value / 1000000 % 1,
        1 => value / 100000 % 10,
        2 => value / 10000 % 10,
        3 => value / 1000 % 10,
        4 => value / 100 % 10,
        5 => value / 10 % 10,
        6 => value / 1 % 10,
        _ => panic!("Not supported"),
    }
}

fn is_valid_p1(i: i32) -> bool {
    if nth(1, &i) > nth(2, &i) || nth(2, &i) > nth(3, &i) || nth(3, &i) > nth(4, &i) || nth(4, &i) > nth(5, &i) || nth(5, &i) > nth(6, &i) {
        return false;
    }
    let number = (1..=6).map(|x| nth(x, &i));
    let mut next_pair = number.clone();
    next_pair.next();
    let mut double = number.zip(next_pair);
    if ! double.any(|(a, b)| a == b) {
        return false;
    }
    true
}

fn is_valid_p2(i: i32) -> bool {
    if nth(1, &i) > nth(2, &i) || nth(2, &i) > nth(3, &i) || nth(3, &i) > nth(4, &i) || nth(4, &i) > nth(5, &i) || nth(5, &i) > nth(6, &i) {
        return false;
    }
    let n: Vec<i32> = (1..=6).map(|x| nth(x, &i)).collect();
    if  n[0] == n[1] && n[1] != n[2] || 
        n[0] != n[1] && n[1] == n[2] && n[2] != n[3] ||
        n[1] != n[2] && n[2] == n[3] && n[3] != n[4] ||
        n[2] != n[3] && n[3] == n[4] && n[4] != n[5] ||
        n[3] != n[4] && n[4] == n[5] {
            true
    } else {
        false
    }
}
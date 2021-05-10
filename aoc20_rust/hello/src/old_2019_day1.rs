use std::fs;
use std::cmp;

pub fn run() {
    part1();
    part2();
}

fn part1(){
    let s = fs::read_to_string("data/2019/1").unwrap();
    let it = s.split('\n');
    let answer: i32 = it.map(|x| cmp::max(0, x.parse::<i32>().unwrap() / 3 - 2)).sum();
    println!("{}", answer);
}

fn part2() {
    let s = fs::read_to_string("data/2019/1").unwrap();
    let it = s.split('\n');
    let fuel: i32 = it.map(|x| calculate_fuel(x.parse::<i32>().unwrap())).sum();
    println!("{}", fuel);
}

fn calculate_fuel(fuel_mass: i32) -> i32 {
    let fuel_for_fuel = fuel_mass / 3 - 2;
    if fuel_for_fuel <= 0 {
        0
    } else {
        calculate_fuel(fuel_for_fuel) + fuel_for_fuel
    }
}

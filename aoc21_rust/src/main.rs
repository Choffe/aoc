mod day1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Must supply which day to run");
    }
    let day: i32 = args[1].parse().expect("Must supply which day to run");
    match day {
        1 => day1::run(),
        _ => panic!("This day is not yet implemented"),
    }
}

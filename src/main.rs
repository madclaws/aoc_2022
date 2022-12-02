mod day1;
mod tools;
use std::env;
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let day = args[1].parse::<i32>().unwrap();
    println!("Advent of Code 2022");
    match day {
        1 => day1::run(),
        _ => panic!("Invalid day"),
    }
}

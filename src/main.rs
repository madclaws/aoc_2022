#![feature(slice_partition_dedup)]
#![feature(control_flow_enum)]
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod tools;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let day = args[1].parse::<i32>().unwrap();
    println!("Advent of Code 2022");
    match day {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        _ => panic!("Invalid day"),
    }
}

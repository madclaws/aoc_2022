use crate::tools;

pub fn run() {
    println!("\n--- Day 4: Camp Cleanup ---\n");

    let input = tools::load_file("/home/madclaws/labs/aoc_2022/data/day4.txt");
    let inclusive_ranges = input
        .split('\n')
        .map(are_ranges_inclusive)
        .filter(|range_results| *range_results)
        .count();

    println!(
        "In how many assignment pairs does one range fully contain the other? => {:?}\n",
        inclusive_ranges
    );

    let overlap_range_count = input
        .split('\n')
        .map(does_ranges_overlap)
        .filter(|range_results| *range_results)
        .count();

    println!(
        "In how many assignment pairs do the ranges overlap? => {}\n",
        overlap_range_count
    );
}

fn are_ranges_inclusive(pairs: &str) -> bool {
    let pair_list = pairs.split(',').collect::<Vec<&str>>();
    let range1 = pair_list[0]
        .split('-')
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let range2 = pair_list[1]
        .split('-')
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    check_inclusive(&range1, &range2) || check_inclusive(&range2, &range1)
}

fn check_inclusive(range1: &[i32], range2: &[i32]) -> bool {
    range1[0] <= range2[0] && range1[1] >= range2[1]
}

fn does_ranges_overlap(pairs: &str) -> bool {
    let pair_list = pairs.split(',').collect::<Vec<&str>>();
    let range1 = pair_list[0]
        .split('-')
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let range2 = pair_list[1]
        .split('-')
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    check_overlapping(&range1, &range2) || check_overlapping(&range2, &range1)
}

fn check_overlapping(range1: &[i32], range2: &[i32]) -> bool {
    range1[0] >= range2[0] && range1[0] <= range2[1]
        || range1[1] >= range2[0] && range1[1] <= range2[1]
}

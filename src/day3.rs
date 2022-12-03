use crate::tools;
pub fn run() {
    println!("\n --- Day 3: Rucksack Reorganization ---\n");

    let input = tools::load_file("/home/madclaws/labs/aoc_2022/data/day3.txt");

    let rucksacks = input.split('\n').collect::<Vec<&str>>();

    let priority_sum = rucksacks
        .iter()
        .map(|rucksack| {
            let compartments = rucksack.split_at(rucksack.len() / 2);
            get_string_diffs(compartments)
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>();

    println!("Priority sum => {}", priority_sum);
}

fn get_string_diffs(compartments: (&str, &str)) -> i32 {
    let (str1, str2) = compartments;
    let str2_list = str2.chars().collect::<Vec<char>>();

    let diffs = str1
        .chars()
        .map(|char1| str2_list.iter().find(|char2| **char2 == char1))
        .flatten()
        .map(|valid_char| {
            if (*valid_char as i32) < 97 {
                ((*valid_char as i32) - 65) + 27
            } else {
                ((*valid_char as i32) - 97) + 1
            }
        })
        .collect::<Vec<i32>>();
    diffs[0]
}

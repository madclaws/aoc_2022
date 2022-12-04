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

    println!(
        "Find the sum of item type that appears in both compartments of each rucksack => {}\n",
        priority_sum
    );

    let chucked_ruck = rucksacks.chunks(3).collect::<Vec<&[&str]>>();

    let chunked_sum = chucked_ruck
        .iter()
        .map(|chucks| get_string_diffs_v2(chucks))
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>();

    println!(
        "Find the sum of item type that corresponds to the badges of each three-Elf group => {:?} \n",
        chunked_sum
    );
}

fn get_string_diffs(compartments: (&str, &str)) -> i32 {
    let (str1, str2) = compartments;
    let str2_list = str2.chars().collect::<Vec<char>>();

    let diffs = str1
        .chars()
        .filter_map(|char1| str2_list.iter().find(|char2| **char2 == char1))
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

fn get_string_diffs_v2(chucked_rucks: &[&str]) -> i32 {
    let str1 = chucked_rucks[0];
    let str2 = chucked_rucks[1];
    let str3 = chucked_rucks[2];

    let str2_list = str2.chars().collect::<Vec<char>>();
    let str3_list = str3.chars().collect::<Vec<char>>();

    let diffs = str1
        .chars()
        .filter_map(|char1| str2_list.iter().find(|char2| **char2 == char1))
        .filter_map(|char2| str3_list.iter().find(|char3| *char2 == **char3))
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

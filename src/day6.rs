use crate::tools;
use std::collections::HashMap;
use std::ops::ControlFlow;
pub fn run() {
    println!("\n--- Day 6: Tuning Trouble --- \n");

    let input = tools::load_file("/home/madclaws/labs/aoc_2022/data/day6.txt");

    let mut input_list = input.split("").collect::<Vec<&str>>();
    input_list.remove(0);
    input_list.remove(input_list.len() - 1);
    let mark = find_marker(&input_list, 4);
    println!("How many characters need to be processed before the first start-of-packet marker is detected? => {}\n", mark);
    let mark = find_marker(&input_list, 14);
    println!("How many characters need to be processed before the first start-of-message marker is detected? => {}\n", mark);
}

fn find_marker(input_list: &[&str], char_count: i32) -> i32 {
    let windows = input_list.windows(char_count as usize);

    let mut count_map: HashMap<&str, i32> = HashMap::new();

    let marked = windows.into_iter().try_fold((0, false), |mark, window| {
        let (mut count, exit) = mark;
        window.iter().fold(0, |acc, c| {
            count_map.entry(c).and_modify(|e| *e += 1).or_insert(1);
            acc
        });
        let mut values_iter = count_map
            .values()
            .filter(|v| **v > 1);

        if exit {
            return ControlFlow::<(i32, bool), (i32, bool)>::Break(mark).break_value();
        }

        if values_iter.next().is_none() {
            count_map.clear();
            return ControlFlow::<(i32, bool), (i32, bool)>::Break((count, true)).break_value();
        }
        count += 1;
        count_map.clear();
        ControlFlow::<(i32, bool), (i32, bool)>::Continue((count, false)).continue_value()
    });
    let (marked_count, _) = marked.unwrap();
    marked_count + char_count
}

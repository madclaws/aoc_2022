use crate::tools;
use std::collections::HashMap;
pub fn run() {
    println!("\n--- Day 6: Tuning Trouble --- \n");

    let input = tools::load_file("/home/madclaws/labs/aoc_2022/data/day6_test.txt");

    println!("{}", input);

    // start of packet -> Different 4 chars

    let mut input_list = input.split("").collect::<Vec<&str>>();
    input_list.remove(0);
    input_list.remove(input_list.len() - 1);

    println!("{:?}", input_list);

    let windows = input_list.windows(4);
    // println!("{:?}", windows.next().unwrap());
    // println!("{:?}", windows.next().unwrap());
    // println!("{:?}", windows.next().unwrap());
    // windows.
    // iter through the windows
    //  if all characters are different
    //  How to check
    //  we can have the marker as accumulator?
    //  starts with 5, then increment by 1
    //
    let mut count_map: HashMap<&str, i32> = HashMap::new();
    
    let marked = windows.into_iter().try_fold(0, |mut mark, window| {
        println!("window {:?} =>", window);
        window.iter().fold(0, |acc, c| {
            count_map.entry(c).and_modify(|e| *e += 1).or_insert(1);
            acc
        });
        println!("{:?}", count_map);
        let values = count_map.values().filter(|v| **v > 1).collect::<Vec<&i32>>();
        println!("values => {:?}", values);
        
        if !values.is_empty() {
            mark += 1;
        } else {
            return Err(())
        }

        count_map.clear();
        Ok(mark)
    });
    println!("{:?}", marked)
    // println!("{:?}, {}", marked, marked.unwrap() + 4);
}

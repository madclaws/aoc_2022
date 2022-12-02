use crate::tools;

pub fn run() {
    println!("\nDay 1: Calorie Counting\n");

    let input = tools::load_file("/home/madclaws/labs/aoc_2022/data/day1_a.txt");

    let elf_calories = input.split("\n\n").collect::<Vec<&str>>();

    let mut calories_sums = elf_calories
        .iter()
        .map(|x| {
            x.split('\n')
                .map(|calorie| calorie.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .iter()
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    calories_sums.sort_by(|a, b| b.cmp(a));
    println!("A) Find the Elf carrying the most Calories.\nHow many total Calories is that Elf carrying? => {:?} Kcal\n", calories_sums[0]);
    println!("B) Find the top three Elves carrying the most Calories.\nHow many Calories are those Elves carrying in total? => {:?} Kcal", calories_sums[0] + calories_sums[1] + calories_sums[2]);
}
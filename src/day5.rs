use crate::tools;
pub fn run() {
    println!("\n--- Day 5: Supply Stacks --- \n");

    let input = tools::load_file("/home/madclaws/labs/aoc_2022/data/day5_test.txt");
    println!("{}", input);

    let stack_and_instructions = input.split("\n\n").collect::<Vec<&str>>();
    
    let stacks_string = stack_and_instructions[0];
    let instructions = stack_and_instructions[1];

    let mut stack_list = stacks_string.split('\n').collect::<Vec<&str>>();

    let stack_count = stack_list.last().unwrap().split("  ").count();

    let mut stacks: Vec<Vec<&str>> = Vec::new();

    stack_list.pop();
    stack_list.iter()
        .fold(0, |acc: i32, stack| {
            let str_stack = stack.split(' ').collect::<Vec<&str>>();
            stacks.push(str_stack);
            acc
        });
    

    // refactor_stack_grid(&stacks);
    let mut new_stack: Vec<Vec<&str>> = Vec::new();
    stacks.iter()
        .fold(0, |mut acc: i32, row| {
            new_stack.push(Vec::new());
            row.iter().fold(0, |mut accu, elem| {
                if elem.is_empty() {
                    accu += 1;
                    if accu == 3 {
                        new_stack[acc as usize].push("");
                        accu = 0;
                    } 
                } else {
                    let elements = elem.split("").collect::<Vec<&str>>();
                    new_stack[acc as usize].push(elements[2])
                }
                accu
            });
            acc += 1;
            acc
        });
    println!("{:?}", new_stack);
}

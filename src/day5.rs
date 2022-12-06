use crate::tools;
pub fn run() {
    println!("\n--- Day 5: Supply Stacks --- \n");

    let input = tools::load_file("/home/madclaws/labs/aoc_2022/data/day5_test.txt");
    println!("{}", input);

    // create vectors for each stacks
    // [D]
    // [N] [C]
    // [Z] [M] [P]

    let stack_and_instructions = input.split("\n\n").collect::<Vec<&str>>();

    println!("{:?}\n", stack_and_instructions);
    
    let stacks_string = stack_and_instructions[0];
    let instructions = stack_and_instructions[1];

    let stack_list = stacks_string.split('\n').collect::<Vec<&str>>();

    let stack_count = stack_list.last().unwrap().split("  ").count();
    println!("{:?}\n", stack_count);

    let mut stacks: Vec<Vec<&str>> = Vec::new();

    println!("{:?}\n", stack_list);
    
    stack_list.iter()
        .fold(0, |acc: i32, stack| {
            let str_stack = stack.split(" ").collect::<Vec<&str>>();
            // str_stack.join(" ");
            stacks.push(str_stack);
            acc
        });
        
    
    println!("{:?}\n", stacks);

    // split by \n
    // skip the first one
    // [N] [C]
    // split by space
    // if ch != "" , then split by '['
    // add to respective stack
    // Create a grid out it stack_count *  stack_count?
}
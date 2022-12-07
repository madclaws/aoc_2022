use crate::tools;
#[derive(Debug)]
struct Instructions {
    from_stack: i32,
    to_stack: i32,
    move_count: i32,
}

impl Instructions {
    pub fn new(from: i32, to: i32, move_count: i32) -> Self {
        Instructions {
            from_stack: from,
            to_stack: to,
            move_count,
        }
    }
}
pub fn run() {
    println!("\n--- Day 5: Supply Stacks --- \n");

    let input = tools::load_file("/home/madclaws/labs/aoc_2022/data/day5.txt");
    // println!("{}", input);

    let stack_and_instructions = input.split("\n\n").collect::<Vec<&str>>();

    let stacks_string = stack_and_instructions[0];
    let instructions = stack_and_instructions[1];
    // println!("{}", instructions);
    let mut stack_list = stacks_string.split('\n').collect::<Vec<&str>>();

    let stack_count = stack_list.last().unwrap().split("  ").count();
    
    println!("stack_count => {}", stack_count);
    
    let mut stacks: Vec<Vec<&str>> = Vec::new();
    
    stack_list.pop();
    // println!("\n{:?}", stack_list);
    
    stack_list.iter().fold(0, |acc: i32, stack| {
        let str_stack = stack.split(" ").collect::<Vec<&str>>();
        stacks.push(str_stack);
        acc
    });

    // println!("STACKS => {:?}", stacks);
    
    // refactor_stack_grid(&stacks);
    let mut new_stack: Vec<Vec<&str>> = Vec::new();
    stacks.iter().fold(0, |mut acc: i32, row| {
        new_stack.push(Vec::new());
        row.iter().fold(0, |mut accu, elem| {
            if elem.is_empty() {
                accu += 1;
                if accu == 4 {
                    new_stack[acc as usize].push("");
                    accu = 0;
                }
            } else {
                accu = 0;
                let elements = elem.split("").collect::<Vec<&str>>();
                // println!("{:?}", elements);
                new_stack[acc as usize].push(elements[2])
            }
            accu
        });
        acc += 1;
        acc
    });

    println!("\n{:?} {}\n", new_stack.len(), new_stack[0].len());

    let instruction_list = create_instruction_set(instructions);

    let mut stack_pointerz: Vec<Vec<&str>> = Vec::new();

    for j in 0..9 {
        let mut single_stack = Vec::new();
        for i in 0..8 {
            if new_stack[i][j] != "" {
                single_stack.push(new_stack[i][j]);
            }
        }
        single_stack.reverse();
        // single_stack.
        stack_pointerz.push(single_stack);
    }

    println!("{:?}", stack_pointerz);

    apply_instructions(&mut stack_pointerz, &instruction_list);

    println!("Stack after applying instructions \n{:?}\n", stack_pointerz);

    let top_stacks = stack_pointerz.iter()
        .map(|c| *c.last().unwrap())
        .collect::<Vec<&str>>()
        .join("");

    println!("TOP STACKS => {:?}", top_stacks);
}

fn create_instruction_set(instructions: &str) -> Vec<Instructions> {
    let instruction_list = instructions.split('\n').collect::<Vec<&str>>();
    instruction_list
        .iter()
        .fold(Vec::new(), |mut instruct_list, elem| {
            let elements = elem.split(' ').collect::<Vec<&str>>();
            instruct_list.push(Instructions::new(
                elements[3].parse::<i32>().unwrap(),
                elements[5].parse::<i32>().unwrap(),
                elements[1].parse::<i32>().unwrap(),
            ));
            instruct_list
        })
}

fn apply_instructions(stack_pointer: &mut Vec<Vec<&str>>, instruction_list: &Vec<Instructions>) {
    instruction_list.iter()
        .fold(0, |acc, instruction| {
            println!("Applying {:?}", instruction);
            let from_stack = &mut stack_pointer[instruction.from_stack as usize  - 1];
            // println!("from stack => {:?}", from_stack);
            let mut popped_items = from_stack.drain(from_stack.len() - (instruction.move_count as usize)..).collect::<Vec<&str>>();
            let to_stack = &mut stack_pointer[instruction.to_stack as usize - 1];
            // println!("to stack => {:?}", to_stack);
            popped_items.reverse();
            to_stack.append(&mut popped_items);
            // println!("NEW STATE------------------ => {:?}", stack_pointer);
            acc
        });
}

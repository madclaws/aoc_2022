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

    let stack_and_instructions = input.split("\n\n").collect::<Vec<&str>>();

    let stacks_string = stack_and_instructions[0];
    let instructions = stack_and_instructions[1];
    let mut stack_list = stacks_string.split('\n').collect::<Vec<&str>>();

    let mut stacks: Vec<Vec<&str>> = Vec::new();

    stack_list.pop();

    stack_list.iter().fold(0, |acc: i32, stack| {
        let str_stack = stack.split(' ').collect::<Vec<&str>>();
        stacks.push(str_stack);
        acc
    });

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
                new_stack[acc as usize].push(elements[2])
            }
            accu
        });
        acc += 1;
        acc
    });

    let instruction_list = create_instruction_set(instructions);

    let mut stack_pointerz: Vec<Vec<&str>> = Vec::new();
    for j in 0..new_stack[0].len() {
        let mut single_stack = Vec::new();
        for i in 0..new_stack.len() {
            if !new_stack[i][j].is_empty() {
                single_stack.push(new_stack[i][j]);
            }
        }
        single_stack.reverse();
        stack_pointerz.push(single_stack);
    }

    let mut stack_pointer_clone = stack_pointerz.clone();

    apply_instructions(&mut stack_pointerz, &instruction_list, "9000");

    let top_stacks = stack_pointerz
        .iter()
        .map(|c| *c.last().unwrap())
        .collect::<Vec<&str>>()
        .join("");

    println!("After the rearrangement procedure completes, what crate ends up on top of each stack on crate 9000? => {:?}", top_stacks);

    apply_instructions(&mut stack_pointer_clone, &instruction_list, "9001");

    let top_stacks = stack_pointer_clone
        .iter()
        .map(|c| *c.last().unwrap())
        .collect::<Vec<&str>>()
        .join("");

    println!("After the rearrangement procedure completes, what crate ends up on top of each stack on crate 9001? => {:?}", top_stacks);
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

fn apply_instructions(
    stack_pointer: &mut [Vec<&str>],
    instruction_list: &[Instructions],
    crate_mover: &str,
) {
    instruction_list.iter().fold(0, |acc, instruction| {
        let from_stack = &mut stack_pointer[instruction.from_stack as usize - 1];
        let mut popped_items = from_stack
            .drain(from_stack.len() - (instruction.move_count as usize)..)
            .collect::<Vec<&str>>();
        let to_stack = &mut stack_pointer[instruction.to_stack as usize - 1];
        if crate_mover == "9000" {
            popped_items.reverse();
        }
        to_stack.append(&mut popped_items);
        acc
    });
}

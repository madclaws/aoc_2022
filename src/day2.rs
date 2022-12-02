use crate::tools;
pub fn run() {
    println!("\n --- Day 2: Rock Paper Scissors --- \n");

    let input = tools::load_file("/home/madclaws/labs/aoc_2022/data/day2.txt");

    let rounds = input.split('\n').collect::<Vec<&str>>();

    let total_score = rounds
        .iter()
        .map(|round| calculate_rps_score(round))
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>();

    println!("What would your total score be if everything goes exactly according to your strategy guide? => {}\n", total_score);

    let total_elf_score = rounds
        .iter()
        .map(|round| calculate_elf_score(round))
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>();

    println!("Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide? => {}\n", total_elf_score);
}

fn calculate_rps_score(round: &str) -> i32 {
    let turns = round.split(' ').collect::<Vec<&str>>();
    let opponent = turns[0];
    let me = turns[1];

    let mut me_score = 0;
    if me == "X" {
        me_score += 1
    } else if me == "Y" {
        me_score += 2
    } else {
        me_score += 3
    }
    let score = match (opponent, me) {
        ("C", "X") | ("B", "Z") | ("A", "Y") => 6,
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        _ => 0,
    };
    me_score += score;
    me_score
}

fn calculate_elf_score(round: &str) -> i32 {
    let turns = round.split(' ').collect::<Vec<&str>>();
    let opponent = turns[0];
    let need_to_do = turns[1];

    let mut me_score = 0;
    if need_to_do == "X" {
        me_score += 0
    } else if need_to_do == "Y" {
        me_score += 3
    } else {
        me_score += 6
    }

    let score = match (opponent, need_to_do) {
        ("A", "Y") => 1,
        ("B", "Y") => 2,
        ("C", "Y") => 3,
        ("A", "X") => 3,
        ("B", "X") => 1,
        ("C", "X") => 2,
        ("A", "Z") => 2,
        ("B", "Z") => 3,
        ("C", "Z") => 1,
        _ => 0,
    };

    me_score += score;
    me_score
}

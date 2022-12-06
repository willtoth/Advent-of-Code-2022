use std::fs;

fn move_stacks(stacks: &mut Vec<Vec<char>>, src: usize, dest: usize, cnt: usize) {
    let range = stacks[src].len() - cnt..;
    let mut tmp = stacks[src].drain(range).rev().collect::<Vec<char>>();
    stacks[dest].append(&mut tmp);
}

fn move_stacks_together(stacks: &mut Vec<Vec<char>>, src: usize, dest: usize, cnt: usize) {
    let range = stacks[src].len() - cnt..;
    let mut tmp = stacks[src].drain(range).collect::<Vec<char>>();
    stacks[dest].append(&mut tmp);
}

fn stock_cargo_row(line: &str, stacks: &mut Vec<Vec<char>>) {
    // Fill in 'blanks' with [-], then tokenize using - as a skip
    line.replace("    ", " [-]")
        .replace(" ", "")
        .split("]")
        .filter_map(|l| l.chars().nth(1))
        .collect::<Vec<char>>()
        .iter()
        .enumerate()
        .filter(|l| *l.1 != '-')
        .for_each(|l| stacks[l.0 + 1].push(*l.1));
}

fn command_to_action(line: &str, stacks: &mut Vec<Vec<char>>, all_at_once: bool) {
    // move 1 from 2 to 1 --> (1, 2, 1)
    let instructions = line
        .replace("move ", "")
        .replace("from ", "")
        .replace("to ", "")
        .split(" ")
        .filter_map(|l| l.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    // [cnt src dest]
    if all_at_once {
        move_stacks_together(stacks, instructions[1], instructions[2], instructions[0]);
    } else {
        move_stacks(stacks, instructions[1], instructions[2], instructions[0]);
    }
}

fn main() {
    // Split out stacks from operations
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    let input = input.split("\n\n").collect::<Vec<&str>>();

    // stack_itr holds an iterator to lines of the stack without the size
    let mut stack_itr = input[0].lines().rev();
    let stacks_cnt = stack_itr.next().unwrap();
    let stacks_cnt = stacks_cnt
        .split(" ")
        .map(|f| f.parse::<i32>().unwrap_or(0))
        .max()
        .unwrap();

    // Preallocate stacks
    let mut stacks_part1: Vec<Vec<char>> = Vec::new();
    stacks_part1.resize((stacks_cnt + 1) as usize, Vec::new());
    let mut stacks_part2: Vec<Vec<char>> = Vec::new();
    stacks_part2.resize((stacks_cnt + 1) as usize, Vec::new());

    // Fill stacks
    stack_itr.for_each(|s| {
        stock_cargo_row(s, &mut stacks_part1);
        stock_cargo_row(s, &mut stacks_part2);
    });

    // Move cargo in stacks Part 1
    input[1]
        .lines()
        .for_each(|l| command_to_action(l, &mut stacks_part1, false));
    input[1]
        .lines()
        .for_each(|l| command_to_action(l, &mut stacks_part2, true));

    println!(
        "Part 1: {}\nPart 2: {}",
        stacks_part1
            .iter()
            .filter_map(|s| s.last())
            .collect::<String>(),
        stacks_part2
            .iter()
            .filter_map(|s| s.last())
            .collect::<String>()
    );
}

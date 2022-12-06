use std::{collections::HashSet, fs};

fn solve(window_size: usize) -> usize {
    fs::read_to_string("input.txt")
        .expect("Unable to read input")
        .as_bytes()
        .windows(window_size)
        .map(|c| c.iter().collect::<HashSet<&u8>>().len())
        .enumerate()
        .find(|f| f.1 == window_size)
        .unwrap()
        .0
        + window_size
}

fn main() {
    println!("Part 1: {}\nPart 2: {}", solve(4), solve(14));
}

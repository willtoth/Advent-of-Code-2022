#![feature(iter_array_chunks)]

use std::{collections::HashSet, fs};

fn score(input: &String) -> i32 {
    input.chars().fold(0, |sum, c| {
        sum + (c as i32 - 96) * ((c as i32 > ('Z' as i32)) as i32)
            + (c as i32 - 38) * ((c as i32 <= ('Z' as i32)) as i32)
    })
}

fn common_items(lhs: &str, rhs: &str) -> String {
    lhs.chars()
        .filter_map(|c| rhs.find(c).map_or(None, |_f| Some(c)))
        .collect::<HashSet<char>>()
        .iter()
        .collect::<String>()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");

    println!(
        "Part 1: {}\nPart 2: {}",
        input.lines().fold(0, |sum, l| {
            sum + score(&common_items(&l[0..l.len() / 2], &l[l.len() / 2..]))
        }),
        input.lines().array_chunks::<3>().fold(0, |sum, l| {
            sum + score(&common_items(&l[2], &common_items(&l[0], &l[1])))
        })
    );
}

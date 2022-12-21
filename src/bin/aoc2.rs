use std::{collections::HashMap, fs};

fn score(s: &String, lookup: HashMap<&str, i32>) -> i32 {
    s.lines().map(|l| lookup.get(l).unwrap_or(&0)).sum::<i32>()
}

fn main() {
    let part1 = HashMap::from([
        ("A X", 3 + 1),
        ("A Y", 6 + 2),
        ("A Z", 0 + 3),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 6 + 1),
        ("C Y", 0 + 2),
        ("C Z", 3 + 3),
    ]);

    let part2 = HashMap::from([
        ("A X", 0 + 3),
        ("A Y", 3 + 1),
        ("A Z", 6 + 2),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 0 + 2),
        ("C Y", 3 + 3),
        ("C Z", 6 + 1),
    ]);

    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    println!("Part 1: {:?}", score(&input, part1));
    println!("Part 2: {:?}", score(&input, part2));
}

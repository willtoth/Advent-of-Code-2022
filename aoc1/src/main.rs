use std::fs;

fn main() {
    // Part 1
    println!(
        "Carrying the most: {:?}",
        fs::read_to_string("input.txt")
            .expect("Unable to read file")
            .split("\n\n")
            .map(|g| g
                .split("\n")
                .map(|i| i.parse::<i32>().unwrap_or(0))
                .sum::<i32>())
            .max()
            .unwrap()
    );

    // Part 2
    let mut result = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .split("\n\n")
        .map(|g| {
            g.split("\n")
                .map(|i| i.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    result.sort();
    result.reverse();

    println!(
        "Three carrying the most: {:?}",
        result[0..3].iter().sum::<i32>()
    );
}

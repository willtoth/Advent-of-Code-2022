#![feature(iter_array_chunks)]
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to open file");
    // Start the '0th' element as 1, then indexing is natural to cycle number
    let mut ops = vec![1; 1];
    let mut accum = 1;
    for line in input.lines() {
        if line.starts_with("noop") {
            ops.push(accum);
        } else {
            let op = line.split(" ").last().unwrap().parse::<i32>().unwrap();
            ops.push(accum);
            ops.push(accum);
            accum += op;
        }
    }

    let mut result = 0;
    for val in ops.iter().enumerate().skip(20).step_by(40) {
        result += (val.0 as i32) * *val.1;
    }
    println!("Part 1: {result}\r\nPart 2:");

    for line in ops.iter().skip(1).array_chunks::<40>() {
        for pixel in line.iter().enumerate() {
            if ((pixel.0 as i32) - **pixel.1).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

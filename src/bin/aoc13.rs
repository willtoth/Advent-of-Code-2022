#![feature(iter_array_chunks)]

use std::fs;
use toml::Value;

// If one element at idx is an array and the other is an integer, make the
// integer into a single element array
fn try_fix_array_mismatch(a: &mut [Value], b: &mut [Value], idx: usize) {
    let left = &mut a[idx];
    let right = &mut b[idx];
    if left.is_array() || right.is_array() {
        if left.is_integer() {
            *left = Value::Array(vec![Value::Integer(left.as_integer().unwrap())]);
            println!(
                "\t- Mixed types; convert left to {:?} and retry comparison",
                left
            );
        }
        if right.is_integer() {
            *right = Value::Array(vec![Value::Integer(right.as_integer().unwrap())]);
            println!(
                "\t- Mixed types; convert right to {:?} and retry comparison",
                right
            );
        }
    }
}

fn print_tabs(idx: usize) {
    for _ in 0..idx {
        print!("\t");
    }
}

fn do_compare(a: &mut [Value], b: &mut [Value], idx: usize) -> Option<bool> {
    print_tabs(idx);
    println!("- Compare {:?} vs {:?}", a, b);

    // Check bounds first
    let a_len = a.len();
    let b_len = b.len();

    if a_len == b_len && idx >= a_len {
        return None;
    }

    if idx >= a_len {
        print_tabs(idx);
        println!("\t- Left side has less elements, so inputs are in the right order");
        return Some(true);
    } else if idx >= b_len {
        print_tabs(idx);
        println!("\t- Right side has less elements, so inputs are in the wrong order");
        return Some(false);
    }

    try_fix_array_mismatch(a, b, idx);

    // Should always have the same type when coming into this function (after checking bounds)
    if !a[idx].same_type(&b[idx]) {
        panic!("Type mismatch");
    }

    if a[idx].is_integer() {
        let left = a[idx].as_integer().unwrap();
        let right = b[idx].as_integer().unwrap();

        print_tabs(idx);
        println!("Compare {left} vs {right}");
        if left < right {
            print_tabs(idx);
            println!("\t- Left side is smaller, so inputs are in the right order");
            return Some(true);
        } else if left > right {
            print_tabs(idx);
            println!("\t- Left side is larger, so inputs are in the wrong order");
            return Some(false);
        }
    } else if a[idx].is_array() {
        let mut left = a[idx].as_array_mut().unwrap();
        let mut right = b[idx].as_array_mut().unwrap();

        let result = do_compare(&mut left, &mut right, 0);
        if result.is_some() {
            return result;
        }
    }

    do_compare(&mut a[idx + 1..], &mut b[idx + 1..], 0)
}

fn compare_elements(a: &mut [Value], b: &mut [Value]) -> bool {
    do_compare(a, b, 0).unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to open file");

    let mut indicies = 1;
    let mut isum = 0;
    let mut lines = Vec::new();

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let s = format!("a = {}", line);
        let toml = s.parse::<Value>().unwrap();
        lines.push(toml["a"].as_array().unwrap().to_owned());
    }

    for line in lines.iter().array_chunks::<2>() {
        println!("== Pair {} ==", indicies);
        let mut left = line[0].to_owned();
        let mut right = line[1].to_owned();
        if compare_elements(&mut left, &mut right) {
            isum += indicies;
        }
        println!("");

        indicies += 1;
    }

    println!("Part 1: {isum}");

    let toml = "a = [[2]]\r\nb = [[6]]".parse::<Value>().unwrap();
    lines.push(toml["a"].as_array().unwrap().to_owned());
    lines.push(toml["b"].as_array().unwrap().to_owned());

    lines.sort_by(|a, b| {
        let mut left = a.to_owned();
        let mut right = b.to_owned();
        if compare_elements(&mut left, &mut right) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let mut idx1 = 0;
    let mut idx2 = 0;
    for line in lines.iter().enumerate() {
        if *line.1 == *toml["a"].as_array().unwrap() {
            idx1 = line.0 + 1;
        }

        if *line.1 == *toml["b"].as_array().unwrap() {
            idx2 = line.0 + 1;
        }
    }

    println!("Part 2: {}", idx1 * idx2);
}

use std::fs;

#[derive(Debug)]
struct Section {
    lower: i32,
    upper: i32,
}

impl Section {
    pub fn overlap_all(&self, other: &Section) -> bool {
        (self.lower >= other.lower && self.upper <= other.upper)
            || (other.lower >= self.lower && other.upper <= self.upper)
    }
    pub fn overlap_some(&self, other: &Section) -> bool {
        (self.lower >= other.lower && self.lower <= other.upper)
            || (self.upper <= other.upper && self.upper >= other.lower)
            || (other.lower >= self.lower && other.lower <= self.upper)
            || (other.upper <= self.upper && other.upper >= self.lower)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");

    let pairs = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|i| {
                    let inner = i
                        .split("-")
                        .map(|j| j.parse().unwrap())
                        .collect::<Vec<i32>>();
                    Section {
                        lower: inner[0],
                        upper: inner[1],
                    }
                })
                .collect::<Vec<Section>>()
        })
        .collect::<Vec<Vec<Section>>>();

    println!(
        "Part 1: {:?}",
        pairs
            .iter()
            .fold(0, |sum, l| sum + (l[0].overlap_all(&l[1]) as i32))
    );

    println!(
        "Part 2: {:?}",
        pairs
            .iter()
            .fold(0, |sum, l| sum + (l[0].overlap_some(&l[1]) as i32))
    );
}

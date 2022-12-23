use aoc2022::geometry::{Grid2d, Point, Rectangle};
use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    //let mut tmp = Grid2d::with_size(8, 50, '.');
    let mut tmp = Grid2d::new('.');

    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let re = Regex::new(r"(x|y)=(-?\d+)").unwrap();
    for line in input.lines() {
        let mut iter = re.captures_iter(line);
        let p1 = Point::new(
            iter.next()
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap(),
            iter.next()
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap(),
        );
        let p2 = Point::new(
            iter.next()
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap(),
            iter.next()
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap(),
        );

        tmp.set_or_insert(p1.x, p1.y, 'S');
        tmp.set_or_insert(p2.x, p2.y, 'B');
    }

    //println!("{:?}", tmp);
    println!("{:#?}", tmp);
}

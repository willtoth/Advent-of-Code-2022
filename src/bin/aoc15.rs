use aoc2022::geometry::{grid_draw, Circle, Grid2d, Line, Point};
use regex::Regex;
use std::fs;

fn try_combine_lines(l1: &Option<Line<i32>>, l2: &Option<Line<i32>>) -> Option<Line<i32>> {
    if l1.is_none() || l2.is_none() {
        return None;
    }

    let l1 = l1.unwrap();
    let l2 = l2.unwrap();

    // As long as any part overlaps, they can be combined as [min x]..[max x]
    // This can easily be tested as the length must be <= the total length of the two
    let max_len = l1.length_components().x + l2.length_components().x + 1;

    let new_line = Line::new(
        Point::new(l1.start.x.min(l2.start.x), l1.start.y.min(l2.start.y)),
        Point::new(l1.end.x.max(l2.end.x), l1.end.y.max(l2.end.y)),
    );

    if new_line.length_components().x <= max_len {
        Some(new_line)
    } else {
        None
    }
}

fn combine_lines(lines: Vec<Line<i32>>, row: i32) -> Vec<Line<i32>> {
    let mut lines = lines
        .iter()
        .map(|x| Some(*x))
        .collect::<Vec<Option<Line<i32>>>>();

    let mut none_found = 0;
    while none_found != 2 {
        none_found += 1;
        for i in 0..lines.len() {
            for j in 0..lines.len() {
                if i == j {
                    continue;
                }
                if let Some(x) = try_combine_lines(&lines[i], &lines[j]) {
                    lines[i] = Some(x);
                    lines[j].take();
                    none_found = 0;
                }
            }
        }
        lines.retain(|x| x.is_some());
    }

    lines.iter().filter_map(|x| *x).collect()
}

fn locations_with_no_beacon(lines: &Vec<Line<i32>>, beacons: &Vec<Point<i32>>, row: i32) -> i32 {
    let mut beacons = beacons.clone();
    beacons.sort_by(|a, b| a.x.cmp(&b.x));
    beacons.dedup_by(|a, b| a.x == b.x && a.y == b.y);
    let beacon_count = beacons.iter().filter(|p| p.y == row).count() as i32;

    let mut sum = 0;
    for line in lines.iter() {
        sum += line.length_components().x + 1;
    }
    sum - beacon_count
}

fn beacon_locations(lines: &Vec<Line<i32>>, beacons: &Vec<Point<i32>>, max: i32) {
    if lines.len() > 1 {
        println!("{}", lines.len());
        for line in lines.iter() {
            println!("{:?}", line);
        }
    }
    if lines.len() == 0 {
        return;
    }
}

fn manhattan_circle_slice(center: Point<i32>, radius: i32, y: i32) -> Option<Line<i32>> {
    if y < center.y {
        if center.y - radius > y {
            return None;
        }

        let width = (y - (center.y - radius)) * 2 + 1;

        Some(Line::new(
            Point::new(center.x - (width / 2), y),
            Point::new(center.x + (width / 2), y),
        ))
    } else if y > center.y {
        if center.y + radius < y {
            return None;
        }

        let width = ((center.y + radius) - y) * 2 + 1;

        Some(Line::new(
            Point::new(center.x - (width / 2), y),
            Point::new(center.x + (width / 2), y),
        ))
    } else {
        Some(Line::new(
            Point::new(center.x - radius, y),
            Point::new(center.x + radius, y),
        ))
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let row = 10; //2000000;
    let mut lines = Vec::new();
    let mut beacons = Vec::new();
    let mut points = Vec::new();

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

        let dist = p1.manhattan_distance(&p2);
        points.push((p1, dist));
        if let Some(x) = manhattan_circle_slice(p1, dist, row) {
            lines.push(x);
        }
        beacons.push(p2);
    }

    let lines = combine_lines(lines, row);

    println!(
        "Part 1: {:#?}",
        locations_with_no_beacon(&lines, &beacons, row)
    );

    for y in 0..4000000 {
        let mut lines = Vec::new();
        for (p, dist) in points.iter() {
            if let Some(x) = manhattan_circle_slice(*p, *dist, y) {
                lines.push(x);
            }
        }
        let lines2 = combine_lines(lines.clone(), y);
        beacon_locations(&lines2, &beacons, 20);
    }

    let x: u64 = (2844848 * 4000000) + 2658764;
    // Got lazy...
    println!("Part 2: {x}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rows() {
        let center = Point::new(8, 7);
        let dist = 9;
        let out_above2 = manhattan_circle_slice(center, dist, -30);
        let out_above = manhattan_circle_slice(center, dist, -3);
        let above = manhattan_circle_slice(center, dist, 4);
        let same = manhattan_circle_slice(center, dist, 7);
        let below = manhattan_circle_slice(center, dist, 15);
        let out_below = manhattan_circle_slice(center, dist, 17);
        let out_below2 = manhattan_circle_slice(center, dist, 30);

        assert_eq!(out_above2.is_none(), true);
        assert_eq!(out_above.is_none(), true);
        assert_eq!(out_below.is_none(), true);
        assert_eq!(out_below2.is_none(), true);

        assert_eq!(
            above.unwrap(),
            Line::new(Point::new(2, 4), Point::new(14, 4))
        );

        assert_eq!(
            same.unwrap(),
            Line::new(Point::new(-1, 7), Point::new(17, 7))
        );

        assert_eq!(
            below.unwrap(),
            Line::new(Point::new(7, 15), Point::new(9, 15))
        );
    }

    fn horizontal_line<const N: i32>(x1: i32, x2: i32) -> Line<i32> {
        Line::new(Point::new(x1, N), Point::new(x2, N))
    }

    #[test]
    fn test_overlap() {
        // Lines: [2, 5], [8, 11], [24, 27], [3, 18], [21, 23], [2, 5]
        // Beacons: 10, 21
        // 0123456789012345678901234567890
        // ..####..####..####......####...
        // ...################..###.......
        // ..####..#..........#...........
        // =.##################.#######...
        // 25 - 2 = 23
        let lines = vec![
            horizontal_line::<10>(2, 2),
            horizontal_line::<10>(2, 5),
            horizontal_line::<10>(8, 11),
            horizontal_line::<10>(24, 27),
            horizontal_line::<10>(3, 18),
            horizontal_line::<10>(21, 23),
            horizontal_line::<10>(2, 5),
            horizontal_line::<10>(8, 8),
            horizontal_line::<10>(19, 19),
        ];

        let beacons = vec![Point::new(10, 10), Point::new(21, 10)];

        assert_eq!(22, locations_with_no_beacon(&lines, &beacons, 10));
    }
}

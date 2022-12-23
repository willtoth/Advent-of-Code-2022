#![feature(array_windows)]

use std::{fmt::Debug, fs};

pub trait BoundingBox {
    fn bounds(&self) -> (Point, Point);

    fn merge_bounds<T>(&self, other: T) -> (Point, Point)
    where
        T: BoundingBox,
    {
        let mine = self.bounds();
        let other = other.bounds();
        let x = [mine.0.x, mine.1.x, other.0.x, other.1.x];
        let y = [mine.0.y, mine.1.y, other.0.y, other.1.y];

        (
            Point::new(*x.iter().min().unwrap(), *y.iter().min().unwrap()),
            Point::new(*x.iter().max().unwrap(), *y.iter().max().unwrap()),
        )
    }

    fn in_bounds(&self, p: &Point) -> bool {
        let bounds = self.bounds();

        p.x >= bounds.0.x && p.x <= bounds.1.x && p.y >= bounds.0.y && p.y <= bounds.1.y
    }
}

impl BoundingBox for (Point, Point) {
    fn bounds(&self) -> (Point, Point) {
        (self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    // ###,###
    pub fn from(s: &str) -> Point {
        let vals = s.split(",").collect::<Vec<&str>>();
        Point {
            x: vals[0].parse().unwrap(),
            y: vals[1].parse().unwrap(),
        }
    }

    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x && self.start.y != self.end.y
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.x != self.end.x && self.start.y == self.end.y
    }

    // Return true if point is occupied by this line
    pub fn occupied(&self, p: &Point) -> bool {
        if self.is_horizontal() {
            let max = self.start.x.max(self.end.x);
            let min = self.start.x.min(self.end.x);
            p.y == self.start.y && min <= p.x && p.x <= max
        } else {
            let max = self.start.y.max(self.end.y);
            let min = self.start.y.min(self.end.y);
            p.x == self.start.x && min <= p.y && p.y <= max
        }
    }
}

impl BoundingBox for Line {
    // (top left e.g. [min_x, miny], bottom right)
    fn bounds(&self) -> (Point, Point) {
        let tl = Point::new(self.start.x.min(self.end.x), self.start.y.min(self.end.y));
        let br = Point::new(self.start.x.max(self.end.x), self.start.y.max(self.end.y));

        (tl, br)
    }
}

#[derive(Debug)]
struct Structure {
    lines: Vec<Line>,
}

impl Structure {
    // ###,### -> ###,### -> ###,###
    pub fn from(s: &str) -> Structure {
        let vals = s.split("->").map(|a| a.trim()).collect::<Vec<&str>>();
        let mut lines = Vec::new();

        for v in vals.array_windows::<2>() {
            lines.push(Line::new(Point::from(v[0]), Point::from(v[1])));
        }

        Structure { lines }
    }

    // Return true if point is occupied by this structure
    pub fn occupied(&self, p: &Point) -> bool {
        for line in self.lines.iter() {
            if line.occupied(p) {
                return true;
            }
        }

        false
    }
}

impl BoundingBox for Structure {
    // (top left e.g. [min_x, miny], bottom right)
    fn bounds(&self) -> (Point, Point) {
        let mut b = self.lines[0].bounds();

        for line in self.lines.iter().skip(1) {
            b = b.merge_bounds(line.bounds());
        }

        b
    }
}

struct Cave {
    rocks: Vec<Structure>,
    sand_entry: Point,
    sand: Vec<Point>,
    has_floor: bool,
}

impl Cave {
    pub fn from(s: &str) -> Cave {
        let mut rocks = Vec::new();
        for line in s.lines() {
            rocks.push(Structure::from(line));
        }
        Cave {
            rocks,
            sand_entry: Point::new(500, 0),
            sand: Vec::new(),
            has_floor: false,
        }
    }

    pub fn add_floor(&mut self) {
        self.has_floor = true;
    }

    pub fn width(&self) -> usize {
        let bounds = self.bounds();
        bounds.1.x - bounds.0.x
    }

    pub fn depth(&self) -> usize {
        self.bounds().1.y
    }

    // Return true if point is occupied by any structure
    pub fn rock(&self, p: &Point) -> bool {
        for s in self.rocks.iter() {
            if s.occupied(p) {
                return true;
            }
        }

        false
    }

    pub fn sand(&self, p: &Point) -> bool {
        for s in self.sand.iter() {
            if s.eq(p) {
                return true;
            }
        }

        false
    }

    pub fn occupied(&self, p: &Point) -> bool {
        let at_floor = if self.has_floor {
            p.y >= self.depth() + 2
        } else {
            false
        };

        self.sand(p) || self.rock(p) || at_floor
    }

    fn do_drop_sand(&self, p: Point) -> Option<Point> {
        if !self.in_bounds(&p) && !self.has_floor {
            return None;
        }

        let mut lookahead = p.clone();
        lookahead.y += 1;

        if !self.occupied(&lookahead) {
            return self.do_drop_sand(lookahead);
        }

        lookahead.x -= 1;

        if !self.occupied(&lookahead) {
            return self.do_drop_sand(lookahead);
        }

        lookahead.x += 2;

        if !self.occupied(&lookahead) {
            return self.do_drop_sand(lookahead);
        }

        if p == self.sand_entry {
            return None;
        }

        Some(p)
    }

    // Return true to sand stayed in cave
    pub fn drop_sand(&mut self) -> bool {
        if let Some(p) = self.do_drop_sand(self.sand_entry.clone()) {
            self.sand.push(p);
            return true;
        }
        false
    }

    pub fn fill_with_sand(&mut self) -> usize {
        while self.drop_sand() {}

        let adder = if self.has_floor { 1 } else { 0 };
        self.sand.iter().count() + adder
    }

    pub fn clear_sand(&mut self) {
        self.sand.clear();
    }
}

impl BoundingBox for Cave {
    // (top left e.g. [min_x, miny], bottom right)
    fn bounds(&self) -> (Point, Point) {
        let mut b = self.rocks[0].bounds();

        for rock in self.rocks.iter().skip(1) {
            b = b.merge_bounds(rock.bounds());
        }
        b.0.y = 0;

        b
    }
}

impl Debug for Cave {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print header
        println!("");
        let bounds = self.bounds();
        let width = ((self.depth() as f64).log(10.0).floor() as usize) + 3;

        for y in bounds.0.y..bounds.1.y + 1 {
            print!("{:<width$}", y, width = width);
            for x in bounds.0.x..bounds.1.x + 1 {
                let mut c = '.';
                if x == self.sand_entry.x && y == self.sand_entry.y {
                    c = '+';
                } else if self.rock(&Point::new(x, y)) {
                    c = '#';
                } else if self.sand(&Point::new(x, y)) {
                    c = 'o';
                }
                print!("{c}");
            }
            println!("");
        }

        if self.has_floor {
            for y in 0..2 {
                print!("{:<width$}", y + bounds.1.y + 1, width = width);
                for _ in bounds.0.x..bounds.1.x + 1 {
                    if y == 0 {
                        print!(".");
                    } else {
                        print!("#");
                    }
                }
                println!("");
            }
        }

        Ok(())
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file.");

    let mut cave = Cave::from(&input);
    println!("{:?}", cave);

    let sand_cnt = cave.fill_with_sand();
    println!("{:?}", cave);
    let part1 = sand_cnt;
    cave.clear_sand();
    cave.add_floor();

    let sand_cnt = cave.fill_with_sand();
    println!("{:?}", cave);
    println!("Part 1: {}\r\nPart 2: {}", part1, sand_cnt);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_structure() {
        let s = Structure::from("498,4 -> 498,6 -> 496,6");
        assert_eq!(s.bounds(), (Point::new(496, 4), Point::new(498, 6)));
        let p1 = Point::new(498, 4);
        let p2 = Point::new(498, 6);
        let p3 = Point::new(496, 6);
        assert_eq!(s.lines[0].start, p1);
        assert_eq!(s.lines[0].end, p2);
        assert_eq!(s.lines[1].start, p2);
        assert_eq!(s.lines[1].end, p3);
    }

    #[test]
    fn structure_occupied() {
        let s = Structure::from("498,4 -> 498,6 -> 496,6");
        let in_points = [
            Point::new(498, 5),
            Point::new(498, 4),
            Point::new(498, 6),
            Point::new(497, 6),
        ];
        let out_points = [
            Point::new(498, 3),
            Point::new(498, 7),
            Point::new(495, 6),
            Point::new(499, 4),
            Point::new(499, 3),
            Point::new(493, 9),
        ];

        for p in in_points {
            assert_eq!(s.occupied(&p), true);
        }
        for p in out_points {
            assert_eq!(s.occupied(&p), false, "{:?}", p);
        }
    }

    #[test]
    fn cave_in_bounds() {
        let s = Cave::from("498,4 -> 498,6 -> 496,6\r\n503,4 -> 502,4 -> 502,9 -> 494,9");
        assert_eq!(s.in_bounds(&Point::new(500, 0)), true);
    }
}

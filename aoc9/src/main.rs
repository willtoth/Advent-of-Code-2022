use std::{collections::HashSet, fmt::Display, fs};

#[derive(Debug, PartialEq)]
enum Direction {
    D,
    U,
    R,
    L,
    S,
    Invalid,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'D' | 'd' => Direction::D,
            'U' | 'u' => Direction::U,
            'R' | 'r' => Direction::R,
            'L' | 'l' => Direction::L,
            'S' | 's' => Direction::S,
            _ => Direction::Invalid,
        }
    }
}

struct Map2d {
    // 2d map of coords with 'true' being visited locations
    // Coordinate system matches problem with increasing to the right
    // and increasing up
    map: HashSet<(i32, i32)>,
    segments: Vec<(i32, i32)>,
}

impl Map2d {
    pub fn new(len: usize) -> Map2d {
        assert!(len > 1, "Must have more than 1 segment");
        let mut hashset = HashSet::new();
        hashset.insert((0, 0));
        Map2d {
            // Start with one location
            map: hashset,
            segments: vec![(0, 0); len],
        }
    }

    pub fn count_visited(&self) -> usize {
        self.map.len()
    }

    fn move_segments(&mut self) {
        for i in 0..self.segments.len() - 1 {
            self.move_segment(i, i + 1);
        }
    }

    fn move_segment(&mut self, leader_idx: usize, follower_idx: usize) {
        let row_diff = self.segments[leader_idx].0 - self.segments[follower_idx].0;
        let col_diff = self.segments[leader_idx].1 - self.segments[follower_idx].1;
        let dist = ((row_diff.pow(2) + col_diff.pow(2)) as f64).sqrt();

        // (sqrt(1^2 + 2^2) = 2.236)
        let do_move = dist > 2.23;

        if row_diff.abs() > 1 || do_move {
            if row_diff.is_positive() {
                self.segments[follower_idx].0 += 1;
            } else {
                self.segments[follower_idx].0 -= 1;
            }
        }

        if col_diff.abs() > 1 || do_move {
            if col_diff.is_positive() {
                self.segments[follower_idx].1 += 1;
            } else {
                self.segments[follower_idx].1 -= 1;
            }
        }
    }

    pub fn step(&mut self, dir: &Direction, steps: i32) {
        assert!(*dir != Direction::S);
        assert!(*dir != Direction::Invalid);
        let mut sign = 1;
        let mut axis = 0;
        match dir {
            Direction::L => {
                sign = -1;
                axis = 1
            }
            Direction::R => {
                sign = 1;
                axis = 1
            }
            Direction::U => sign = 1,
            Direction::D => sign = -1,
            Direction::S | Direction::Invalid => {}
        }

        for _ in 0..steps {
            if axis == 0 {
                self.segments[0].0 += sign;
            } else {
                self.segments[0].1 += sign;
            }
            self.move_segments();
            self.map.insert(*self.segments.last().unwrap());
        }
    }
}

impl Display for Map2d {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows_max = self.map.iter().map(|p| p.0).max().unwrap() + 5;
        let rows_min = self.map.iter().map(|p| p.0).min().unwrap() - 5;
        let cols_max = self.map.iter().map(|p| p.1).max().unwrap() + 5;
        let cols_min = self.map.iter().map(|p| p.1).min().unwrap() - 5;

        for i in (rows_min..rows_max).rev() {
            for j in cols_min..cols_max {
                let mut c = ".";
                let letter = self
                    .segments
                    .iter()
                    .enumerate()
                    .map(|l| {
                        let val = if l.0 == 0 {
                            String::from("H")
                        } else {
                            l.0.to_string()
                        };
                        (val, l.1)
                    })
                    .find(|f| *f.1 == (i, j));
                if letter.is_some() {
                    c = &letter.as_ref().unwrap().0;
                } else if (i, j) == (0, 0) {
                    c = "s";
                } else if self.map.contains(&(i, j)) {
                    c = "#";
                }
                print!("{c} ");
            }
            print!("\r\n");
        }

        Ok(())
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to open file");
    let cmds = input
        .lines()
        .map(|l| {
            let items = l.split(" ").collect::<Vec<&str>>();
            (
                items[0].chars().nth(0).unwrap().into(),
                items[1].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(Direction, i32)>>();

    let mut map_pt1 = Map2d::new(2);
    let mut map_pt2 = Map2d::new(10);

    for cmd in cmds {
        map_pt1.step(&cmd.0, cmd.1);
        map_pt2.step(&cmd.0, cmd.1);
    }

    println!(
        "Part 1: {}\r\nPart 2: {}",
        map_pt1.count_visited(),
        map_pt2.count_visited()
    );
}

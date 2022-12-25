use std::{fmt::Debug, ops::Rem, vec};

use crate::geometry::{BoundingBox, Point, Rectangle};

pub struct GridRowIterator<'a, T: Copy + Clone> {
    grid: &'a Grid2d<T>,
    row: i32,
    col: i32,
}

impl<'a, T: Copy + Clone> Iterator for GridRowIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.grid.index(self.col, self.row).ok();
        self.col += 1;

        val
    }
}

pub struct Grid2d<T: Copy + Clone> {
    grid: Vec<Vec<T>>,
    coord_top_left: Point<i32>,
    pub default: T,
    bounds: Option<Rectangle<i32>>,
}

impl<T: Copy + Clone> Grid2d<T> {
    pub fn new(default: T) -> Grid2d<T> {
        Grid2d {
            grid: vec![vec![default; 1]; 1],
            coord_top_left: Point::new(0, 0),
            default,
            bounds: None,
        }
    }

    pub fn index(&self, x: i32, y: i32) -> Result<&T, ()> {
        if !self.in_bounds(&Point::new(x, y)) {
            return Err(());
        }
        Ok(&self.grid[(y - self.coord_top_left.y) as usize][(x - self.coord_top_left.x) as usize])
    }

    pub fn index_mut(&mut self, x: i32, y: i32) -> &mut T {
        &mut self.grid[(y - self.coord_top_left.y) as usize][(x - self.coord_top_left.x) as usize]
    }

    pub fn row_iter(&self, row: i32) -> GridRowIterator<T> {
        GridRowIterator {
            grid: self,
            row,
            col: self.coord_top_left.x,
        }
    }

    pub fn set_or_insert(&mut self, x: i32, y: i32, value: T) {
        let p = Point::new(x, y);
        let bounds = self.bounds();

        if let Some(a) = self.bounds {
            if !a.in_bounds(&Point::new(x, y)) {
                return;
            }
        }

        if !self.in_bounds(&p) {
            // Add columns before
            if x < bounds.tl.x {
                let num_to_add = (bounds.tl.x - x) as usize;

                for i in 0..bounds.height() {
                    let mut new_vec = vec![self.default; num_to_add];
                    new_vec.append(&mut self.grid[i as usize]);
                    self.grid[i as usize] = new_vec;
                }

                // Move first point
                self.coord_top_left.x = x;
            }
            let bounds = self.bounds();

            // Add rows before
            if y < bounds.tl.y {
                let num_to_add = (bounds.tl.y - y) as usize;
                let mut new_row = vec![vec![self.default; bounds.width() as usize]; num_to_add];
                new_row.append(&mut self.grid);

                self.grid = new_row;

                // Move first point
                self.coord_top_left.y = y;
            }
            let bounds = self.bounds();

            // Add columns after
            if x >= bounds.br.x {
                let num_to_add = (x - bounds.br.x + 1) as usize;

                for i in 0..bounds.height() {
                    for _ in 0..num_to_add {
                        self.grid[i as usize].push(self.default.clone());
                    }
                }
            }
            let bounds = self.bounds();

            // Add rows after
            if y >= bounds.br.y {
                let num_to_add = (y - bounds.br.y + 1) as usize;

                for _ in 0..num_to_add {
                    self.grid.push(vec![self.default; bounds.width() as usize]);
                }
            }
        }

        self.grid[(y - self.coord_top_left.y) as usize][(x - self.coord_top_left.x) as usize] =
            value
    }
}

impl<T: Clone + Copy> Grid2d<T> {
    pub fn with_size(width: usize, height: usize, start_val: T) -> Grid2d<T> {
        Grid2d {
            grid: vec![vec![start_val; width]; height],
            coord_top_left: Point::new(0, 0),
            default: start_val,
            bounds: None,
        }
    }

    pub fn with_coordinates(coords: Rectangle<i32>, start_val: T) -> Grid2d<T> {
        Grid2d {
            grid: vec![vec![start_val; coords.width() as usize]; coords.height() as usize],
            coord_top_left: coords.tl,
            default: start_val,
            bounds: None,
        }
    }

    pub fn set_max_bounds(&mut self, bounds: Rectangle<i32>) {
        self.bounds = Some(bounds);
    }
}

impl<T: Copy> BoundingBox<i32> for Grid2d<T> {
    fn bounds(&self) -> Rectangle<i32> {
        let x_len = if self.grid.len() == 0 {
            0
        } else {
            self.grid[0].len()
        } as i32;
        Rectangle::new(
            self.coord_top_left,
            self.coord_top_left + Point::new(x_len, self.grid.len() as i32),
        )
    }
}

fn get_digit(x: i32, digit: usize) -> char {
    let arr = x.abs().to_string().chars().rev().collect::<Vec<char>>();

    if digit >= arr.len() {
        return '0';
    } else {
        return arr[digit];
    }
}

impl<T: ToString + Copy> Debug for Grid2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bounds = self.bounds();
        let width = ((bounds.height() as f64).log(10.0).floor() as usize) + 3;
        let pad_height = ((bounds.width() as f64).log(10.0).floor() as usize) + 1;

        if f.alternate() {
            // Print header
            for i in (0..pad_height + 1).rev() {
                print!("{:width$}", " ", width = width);
                for j in bounds.tl.x..bounds.br.x {
                    if j.rem(5) == 0 {
                        let digit = get_digit(j, i);
                        if i == pad_height && j.is_negative() {
                            print!("-");
                        } else if digit == '0' && i != 0 {
                            print!(" ");
                        } else {
                            print!("{digit}");
                        }
                    } else {
                        print!(" ");
                    }
                }
                println!("");
            }
        }

        for y in bounds.tl.y..bounds.br.y {
            if f.alternate() {
                print!("{:<width$}", y, width = width);
            }

            for x in bounds.tl.x..bounds.br.x {
                print!("{}", self.index(x, y).unwrap().to_string());
            }
            println!("");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_creation() {
        let grid = Grid2d::with_size(10, 10, '.');
        assert_eq!(grid.grid[9][9], '.');

        let r = Rectangle::new(Point { x: 5, y: 5 }, Point { x: -5, y: -5 });

        let grid = Grid2d::with_coordinates(r, '!');
        assert_eq!(grid.grid[9][9], '!');
    }

    #[test]
    fn set_or_insert() {
        let mut grid = Grid2d::with_size(10, 10, '.');
        assert_eq!(*grid.index(5, 5).unwrap(), '.');
        grid.set_or_insert(5, 5, '#');
        assert_eq!(*grid.index(5, 5).unwrap(), '#');

        grid.set_or_insert(10, 10, 'w');
        assert_eq!(*grid.index(10, 10).unwrap(), 'w');

        grid.set_or_insert(15, 15, '8');
        assert_eq!(*grid.index(15, 15).unwrap(), '8');
        assert_eq!(*grid.index(12, 12).unwrap(), '.');

        grid.set_or_insert(-8, -3, 'M');
        assert_eq!(*grid.index(-8, -3).unwrap(), 'M');
        assert_eq!(*grid.index(-2, -1).unwrap(), '.');
        assert_eq!(grid.coord_top_left, Point::new(-8, -3));
        assert_eq!(grid.index(-9, -4), Err(()));
    }
}

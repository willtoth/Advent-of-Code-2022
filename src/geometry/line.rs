use crate::geometry::{Num, Point};
use num_traits::{PrimInt, Signed};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line<T>
where
    T: Num,
{
    pub start: Point<T>,
    pub end: Point<T>,
}

impl<T> Line<T>
where
    T: Num,
{
    pub fn new(start: Point<T>, end: Point<T>) -> Line<T> {
        Line { start, end }
    }
}

impl<T: Num + AddAssign<i32>> Line<T> {
    pub fn length_components(&self) -> Point<T> {
        let x = (self.start.x).max(self.end.x) - (self.start.x).min(self.end.x);
        let y = (self.start.y).max(self.end.y) - (self.start.y).min(self.end.y);
        Point::new(x, y)
    }
}

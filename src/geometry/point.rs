use crate::geometry::Num;
use num_traits::{PrimInt, Signed};
use std::{fmt::Debug, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<T>
where
    T: Num,
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Num,
{
    // ###,###
    pub fn from(s: &str) -> Result<Point<T>, &'static str> {
        let vals = s.split(",").collect::<Vec<&str>>();

        Ok(Point {
            // Remove dependency on <T as FromStr>::Err
            x: vals[0].parse::<T>().map_err(|_| "Unable to parse string")?,
            y: vals[1].parse::<T>().map_err(|_| "Unable to parse string")?,
        })
    }

    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl<T: Num> Point<T>
where
    f64: From<T>,
{
    pub fn distance(&self, other: &Point<T>) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        <T as Into<f64>>::into((dx * dx) + (dy * dy)).sqrt()
    }
}

impl<T: Num + PrimInt + Signed> Point<T> {
    pub fn manhattan_distance(&self, other: &Point<T>) -> T {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();

        dx + dy
    }
}

impl<T: Num> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

use crate::geometry::Num;
use std::{fmt::Debug, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl<T: Num> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

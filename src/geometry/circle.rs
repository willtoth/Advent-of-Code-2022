use crate::geometry::{BoundingBox, Num, Point};
use std::fmt::Debug;

use super::Rectangle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle<T: Num> {
    pub center: Point<T>,
    pub radius: T,
}

impl<T: Num> Circle<T> {
    pub fn new(center: Point<T>, radius: T) -> Circle<T> {
        Circle { center, radius }
    }
}

impl<T: Num> BoundingBox<T> for Circle<T> {
    fn bounds(&self) -> Rectangle<T> {
        Rectangle::new(
            Point::new(self.center.x - self.radius, self.center.y + self.radius),
            Point::new(self.center.x + self.radius, self.center.y - self.radius),
        )
    }
}

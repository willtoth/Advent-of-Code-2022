use crate::geometry::{BoundingBox, Filled, Num, Point, Rectangle};
use std::fmt::Debug;

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

impl<T: Num> Filled for Circle<T> {}

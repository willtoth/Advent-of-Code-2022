use crate::geometry::{BoundingBox, Num, Point};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle<T: Num> {
    pub tl: Point<T>,
    pub br: Point<T>,
}

impl<T: Num> Rectangle<T> {
    pub fn new(tl: Point<T>, br: Point<T>) -> Rectangle<T> {
        Rectangle { tl, br }
    }

    pub fn width(&self) -> T {
        self.tl.x.max(self.br.x) - self.tl.x.min(self.br.x)
    }

    pub fn height(&self) -> T {
        self.tl.y.max(self.br.y) - self.tl.y.min(self.br.y)
    }
}

impl<T: Num> BoundingBox<T> for Rectangle<T> {
    fn bounds(&self) -> Rectangle<T> {
        *self
    }
}

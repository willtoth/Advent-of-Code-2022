use crate::geometry::{Num, Point, Rectangle};

pub trait BoundingBox<T: Num> {
    fn bounds(&self) -> Rectangle<T>;

    fn merge_bounds<B>(&self, other: B) -> Rectangle<T>
    where
        B: BoundingBox<T>,
    {
        let mine = self.bounds();
        let other = other.bounds();
        let x = [mine.tl.x, mine.br.x, other.tl.x, other.br.x];
        let y = [mine.tl.y, mine.br.y, other.tl.y, other.br.y];

        Rectangle::new(
            Point::new(*x.iter().min().unwrap(), *y.iter().min().unwrap()),
            Point::new(*x.iter().max().unwrap(), *y.iter().max().unwrap()),
        )
    }

    fn in_bounds(&self, p: &Point<T>) -> bool {
        let bounds = self.bounds();

        p.x >= bounds.tl.x && p.x < bounds.br.x && p.y >= bounds.tl.y && p.y < bounds.br.y
    }
}

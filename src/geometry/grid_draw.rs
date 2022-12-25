use num_traits::PrimInt;

use crate::geometry::{Circle, Grid2d, Line, Num};

pub fn draw_line<B: Copy + Clone, T: Num + PrimInt, F>(grid: &mut Grid2d<B>, line: &Line<T>, f: F)
where
    i32: From<T>,
    F: Fn(&B) -> B,
{
    let min = Into::<i32>::into(line.start.x.min(line.end.x));
    let max = Into::<i32>::into(line.start.x.max(line.end.x));
    for x in min..max + 1 {
        let y = Into::<i32>::into(line.start.y);
        let v = grid.index(x, y).unwrap_or(&grid.default);
        grid.set_or_insert(x, y, f(v));
    }
}

pub fn draw_manhattan_circle<B: Copy + Clone, T: Num + PrimInt, F>(
    grid: &mut Grid2d<B>,
    circle: Circle<T>,
    f: F,
) where
    i32: From<T>,
    F: Fn(&B) -> B,
{
    let min_y = Into::<i32>::into(circle.center.y - circle.radius);
    let max_y = Into::<i32>::into(circle.center.y + circle.radius);

    let mut cnt = 0;
    for y in min_y..max_y + 1 {
        let range =
            Into::<i32>::into(circle.center.x) - cnt..Into::<i32>::into(circle.center.x) + cnt + 1;

        for x in range {
            let v = grid.index(x, y).unwrap_or(&grid.default);
            grid.set_or_insert(x, y, f(v));
        }

        if y >= Into::<i32>::into(circle.center.y) {
            cnt -= 1;
        } else {
            cnt += 1;
        }
    }
}

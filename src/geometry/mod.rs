pub mod bounding_box;
pub mod circle;
pub mod grid2d;
pub mod grid_draw;
pub mod line;
pub mod point;
pub mod rectangle;

// TODO: Better errors

use num_traits::NumOps;
use std::{fmt::Debug, str::FromStr};
pub trait Num: NumOps + Debug + Copy + FromStr + Sized + PartialOrd + Ord + Eq + PartialEq {}
impl<T> Num for T where
    T: NumOps + Debug + Copy + FromStr + Sized + PartialOrd + Ord + Eq + PartialEq
{
}

pub trait Filled {}

pub use crate::geometry::bounding_box::*;
pub use crate::geometry::circle::*;
pub use crate::geometry::grid2d::*;
pub use crate::geometry::line::*;
pub use crate::geometry::point::*;
pub use crate::geometry::rectangle::*;

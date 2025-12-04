use crate::common::{direction::Direction, grid::Point};

#[derive(Debug, Clone, Copy, Default)]
pub struct Guard {
    point: Point,
    direction: Direction,
}

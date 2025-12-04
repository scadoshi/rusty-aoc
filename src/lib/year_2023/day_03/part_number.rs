use crate::common::grid::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PartNumber {
    pub value: usize,
    pub start: Point,
}

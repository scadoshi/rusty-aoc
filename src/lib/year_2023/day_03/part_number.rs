use crate::year_2023::day_03::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PartNumber {
    pub value: usize,
    pub start: Point,
}

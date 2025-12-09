use crate::common::grid::Point;

pub fn get_input() -> Vec<Point> {
    include_str!("input.txt")
        .lines()
        .map(|x| Point::from(x).pivoted())
        .collect()
}

use crate::common::grid::Point;

pub fn get_input() -> Vec<Point> {
    include_str!("input.txt").lines().map(Point::from).collect()
}

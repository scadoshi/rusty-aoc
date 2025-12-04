use crate::common::direction::Direction;

pub fn get_input() -> Vec<Direction> {
    include_str!("input.txt")
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(Direction::from)
        .collect()
}

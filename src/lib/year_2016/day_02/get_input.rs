use crate::common::direction::Direction;

pub fn get_input() -> Vec<Vec<Direction>> {
    include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(Direction::from).collect())
        .collect()
}

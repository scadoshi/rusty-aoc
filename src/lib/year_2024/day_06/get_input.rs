use crate::common::grid::Grid;

pub fn get_input() -> Grid<char> {
    include_str!("input.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

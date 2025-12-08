use crate::common::grid::Grid;

pub fn get_input() -> Grid<&'static u8> {
    include_str!("input.txt")
        .lines()
        .map(|l| l.as_bytes().into_iter().collect())
        .collect()
}

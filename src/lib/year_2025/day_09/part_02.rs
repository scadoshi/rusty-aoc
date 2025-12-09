use crate::common::{
    direction::Direction,
    grid::{Grid, Point},
};
use std::{collections::HashSet, ops::RangeInclusive};

pub fn part_02(input: &[Point]) -> usize {
    let border: HashSet<Point> = input
        .windows(2)
        .filter_map(|w| w[0].axis_line_to(w[1]))
        .flatten()
        .collect();

    let max_row = input.iter().map(|p| p.row).max().unwrap();
    let max_col = input.iter().map(|p| p.col).max().unwrap();

    let mut grid: Grid<(bool, usize)> = (0..=max_row)
        .map(|_| (0..=max_col).map(|_| (false, 0)).collect())
        .collect();

    0
}

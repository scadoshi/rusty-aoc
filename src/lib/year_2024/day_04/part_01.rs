use crate::{common::grid::Grid, year_2024::day_04::xmas_at_point::XmasAtPoint};

pub fn part_01(input: &Grid<char>) -> usize {
    input
        .to_points()
        .into_iter()
        .map(|p| input.xmas_at_point(p))
        .sum()
}

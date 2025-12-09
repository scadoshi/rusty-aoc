use crate::{common::grid::Grid, year_2024::day_04::cross_mas_at_point::CrossMasAtPoint};

pub fn part_02(input: &Grid<char>) -> usize {
    input
        .to_points()
        .into_iter()
        .filter(|p| input.cross_mas_at_point(*p))
        .count()
}

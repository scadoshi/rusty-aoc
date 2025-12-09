use crate::{common::grid::Grid, year_2023::day_03::grid::GridOpsExt};

pub fn part_02(input: &Grid<char>) -> usize {
    input
        .to_points()
        .iter()
        .filter_map(|p| input.get_gear_ratio_at_point(*p))
        .sum()
}

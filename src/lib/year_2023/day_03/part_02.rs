use crate::year_2023::day_03::grid::{Grid, GridOperations};

pub fn part_02(input: &Grid) -> usize {
    input
        .points()
        .iter()
        .filter_map(|p| input.get_gear_ratio_at_point(*p))
        .sum()
}

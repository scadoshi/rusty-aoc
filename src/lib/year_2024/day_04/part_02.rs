use crate::year_2024::day_04::{
    cross_mas_at_point::CrossMasAtPoint,
    grid::{Grid, GridOperations},
};

pub fn part_02(input: &Grid) -> usize {
    input
        .to_points()
        .into_iter()
        .filter(|p| input.cross_mas_at_point(*p))
        .count()
}

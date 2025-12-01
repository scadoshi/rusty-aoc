use std::collections::HashSet;

use crate::year_2023::day_03::{
    grid::{Grid, GridOperations},
    part_number::PartNumber,
};

pub fn part_01(input: &Grid) -> usize {
    input
        .points()
        .iter()
        .filter_map(|p| input.get_part_number_at_point(*p))
        .collect::<HashSet<PartNumber>>()
        .iter()
        .map(|pn| pn.value)
        .sum()
}

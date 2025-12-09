use crate::{
    common::grid::Grid,
    year_2023::day_03::{grid::GridOpsExt, part_number::PartNumber},
};
use std::collections::HashSet;

pub fn part_01(input: &Grid<char>) -> usize {
    input
        .to_points()
        .iter()
        .filter_map(|p| input.get_part_number_at_point(*p))
        .collect::<HashSet<PartNumber>>()
        .iter()
        .map(|pn| pn.value)
        .sum()
}

use crate::{common::grid::Grid, year_2020::day_03::collisions_at_slope::CollisionsAtSlope};

pub fn part_01(input: &Grid<char>) -> usize {
    input.collisions_at_slope::<1, 3>()
}

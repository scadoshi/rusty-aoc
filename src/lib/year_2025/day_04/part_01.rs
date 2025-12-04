use crate::{common::grid::Grid, year_2025::day_04::removeable::Removeable};

pub fn part_01(input: &Grid<char>) -> usize {
    input.removeable().len()
}

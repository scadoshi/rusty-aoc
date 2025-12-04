use crate::{
    common::grid::{Grid, GridOps},
    year_2025::day_04::removeable::Removeable,
};

pub fn part_02(input: &Grid<char>) -> usize {
    let mut grid = input.clone();
    let mut removeable = grid.removeable();
    let mut total_removed = 0;
    while !removeable.is_empty() {
        for point in removeable.iter() {
            grid.set_value_at_point('x', *point);
            total_removed += 1;
        }
        removeable = grid.removeable();
    }
    total_removed
}

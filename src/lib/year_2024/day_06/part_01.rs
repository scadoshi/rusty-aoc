use crate::{common::grid::Grid, year_2024::day_06::guard::GuardOps};

pub fn part_01(input: &Grid<char>) -> usize {
    input.simulate_guard().visited.len()
}

use crate::{common::grid::Grid, year_2024::day_06::guard::GuardOps};

pub fn part_02(input: &Grid<char>) -> usize {
    let path = input.simulate_guard();
    path.visited
        .iter()
        .map(|point| {
            let mut new_grid = input.clone();
            new_grid.place_obstacle(*point);
            usize::from(new_grid.simulate_guard().infinite_loop_encountered)
        })
        .sum()
}

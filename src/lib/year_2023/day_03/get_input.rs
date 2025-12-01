use crate::year_2023::day_03::grid::Grid;

pub fn get_input() -> Grid {
    include_str!("input.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}

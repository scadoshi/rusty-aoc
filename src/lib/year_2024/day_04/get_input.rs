use crate::year_2024::day_04::grid::Grid;

pub fn get_input() -> Grid {
    include_str!("input.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}

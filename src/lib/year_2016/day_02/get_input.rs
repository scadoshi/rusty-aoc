use crate::year_2016::day_02::direction::Direction;

pub fn get_input() -> Vec<Vec<Direction>> {
    include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(Direction::from).collect())
        .collect()
}

use crate::year_2015::day_03::direction::Direction;

pub fn get_input() -> Vec<Direction> {
    include_str!("input.txt")
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(Direction::from)
        .collect()
}

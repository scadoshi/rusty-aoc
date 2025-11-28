use crate::year_2015::day_03::{direction::Direction, point::Point};
use std::collections::HashSet;

pub fn part_01(input: &[Direction]) -> usize {
    input
        .iter()
        .fold(
            (HashSet::from([Point::origin()]), Point::origin()),
            |(mut visited, mut position), direction| {
                position.traverse(*direction);
                visited.insert(position);
                (visited, position)
            },
        )
        .0
        .len()
}

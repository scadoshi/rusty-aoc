use crate::common::{cartesian_point::Point, direction::Direction};
use std::collections::HashSet;

pub fn part_01(input: &[Direction]) -> usize {
    input
        .iter()
        .fold(
            (HashSet::from([Point::new()]), Point::new()),
            |(mut visited, mut position), direction| {
                position.step(*direction);
                visited.insert(position);
                (visited, position)
            },
        )
        .0
        .len()
}

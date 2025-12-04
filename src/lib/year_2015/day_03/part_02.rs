use crate::common::{cartesian_point::Point, direction::Direction};
use std::collections::HashSet;

pub fn part_02(input: &[Direction]) -> usize {
    input
        .iter()
        .enumerate()
        .fold(
            (HashSet::from([Point::new()]), Point::new(), Point::new()),
            |(mut visited, mut santa, mut robo_santa), (i, direction)| {
                if i % 2 == 0 {
                    robo_santa.step(*direction);
                    visited.insert(robo_santa);
                } else {
                    santa.step(*direction);
                    visited.insert(santa);
                }
                (visited, santa, robo_santa)
            },
        )
        .0
        .len()
}

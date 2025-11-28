use crate::year_2015::day_03::{direction::Direction, point::Point};
use std::collections::HashSet;

pub fn part_02(input: &[Direction]) -> usize {
    input
        .iter()
        .enumerate()
        .fold(
            (
                HashSet::from([Point::origin()]),
                Point::origin(),
                Point::origin(),
            ),
            |(mut visited, mut santa, mut robo_santa), (i, direction)| {
                if i % 2 == 0 {
                    robo_santa.traverse(*direction);
                    visited.insert(robo_santa);
                } else {
                    santa.traverse(*direction);
                    visited.insert(santa);
                }
                (visited, santa, robo_santa)
            },
        )
        .0
        .len()
}

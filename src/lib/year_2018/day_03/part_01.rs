use crate::{common::grid::Point, year_2018::day_03::claim::Claim};
use std::collections::HashMap;

pub fn part_01(input: &[Claim]) -> usize {
    input
        .iter()
        .fold(HashMap::<Point, usize>::new(), |mut map, claim| {
            claim.to_points().iter().for_each(|point| {
                *map.entry(*point).or_default() += 1;
            });
            map
        })
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .count()
}

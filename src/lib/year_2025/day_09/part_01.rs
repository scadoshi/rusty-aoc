use crate::{
    common::grid::Point,
    year_2025::day_09::{
        for_each_combination::ForEachCombination, rectangle::OfRectangleWithOtherCorner,
    },
};

pub fn part_01(input: &[Point]) -> Option<usize> {
    let mut max = 0;
    input.for_each_combination(|p1, p2| max = max.max(p1.area(*p2)));
    Some(max)
}

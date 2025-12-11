use crate::{
    common::grid::Point,
    year_2025::day_09::{
        for_each_combination::ForEachCombination, rectangle::OfRectangleWithOtherCorner,
    },
};
use std::{collections::HashMap, ops::RangeInclusive};

pub fn part_02(input: &[Point]) -> Option<usize> {
    let boundary = (0..input.len())
        .filter_map(|p| {
            let p1 = input.get(p)?;
            let p2 = input.get((p + 1) % input.len())?;
            Some((p1, p2))
        })
        .fold(
            HashMap::<usize, Vec<RangeInclusive<usize>>>::new(),
            |mut boundary, (p1, p2)| {
                (p1.row.min(p2.row)..=p1.row.max(p2.row)).for_each(|row| {
                    boundary
                        .entry(row)
                        .or_default()
                        .push(p1.col.min(p2.col)..=p1.col.max(p2.col))
                });
                boundary
            },
        );
    let mut max = 0;
    input.for_each_combination(|p1, p2| {
        if p1
            .perimeter_ranges(*p2)
            .iter()
            .all(|(row, perimeter_range)| {
                let Some(boundary_ranges) = boundary.get(row) else {
                    return false;
                };
                boundary_ranges.iter().any(|bound_range| {
                    bound_range.contains(perimeter_range.start())
                        && bound_range.contains(perimeter_range.end())
                })
            })
        {
            max = max.max(p1.area(*p2));
        }
    });
    Some(max)
}

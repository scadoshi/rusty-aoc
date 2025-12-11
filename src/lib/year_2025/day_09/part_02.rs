use crate::{
    common::grid::Point,
    year_2025::day_09::{
        for_each_combination::ForEachCombination, rectangle::OfRectangleWithOtherCorner,
    },
};
use std::{collections::HashMap, ops::RangeInclusive};

pub fn part_02(input: &[Point]) -> Option<usize> {
    let started_at = std::time::Instant::now();
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
    let total_rectangles: usize = input
        .iter()
        .enumerate()
        .map(|(i, _)| input.iter().skip(i + 1).count())
        .sum();
    let mut rectangles_processed: usize = 0;
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
        rectangles_processed += 1;
        let total_time = started_at.elapsed();
        let avg_time = total_time / rectangles_processed as u32;
        let rem_time = avg_time.mul_f32((total_rectangles - rectangles_processed) as f32);
        let percentage = rectangles_processed as f32 / total_rectangles as f32 * 100.0;
        if rectangles_processed % 1_000 == 0 {
            println!(
                "{}/{} | {:.2}% | total: {:?} | avg: {:?} | rem: {:?}",
                rectangles_processed, total_rectangles, percentage, total_time, avg_time, rem_time
            );
        }
    });
    Some(max)
}

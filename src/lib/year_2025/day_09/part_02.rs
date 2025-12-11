use crate::{
    common::grid::Point,
    year_2025::day_09::{
        boundary::Boundary, for_each_combination::ForEachCombination,
        rectangle::OfRectangleWithOtherCorner,
    },
};

pub fn part_02(input: &[Point]) -> Option<usize> {
    let started_at = std::time::Instant::now();
    let boundary = input.boundary();
    let mut max = 0;
    let total_rectangles: usize = input
        .iter()
        .enumerate()
        .map(|(i, _)| input.iter().skip(i + 1).count())
        .sum();
    let mut rectangles_processed: usize = 0;
    input.for_each_combination(|p1, p2| {
        if p1.perimeter(*p2).iter().all(|(row, perimeter_ranges)| {
            let Some(boundary_ranges) = boundary.get(row) else {
                return false;
            };
            boundary_ranges.iter().any(|br| {
                perimeter_ranges
                    .iter()
                    .all(|pr| br.contains(pr.start()) && br.contains(pr.end()))
            })
        }) {
            max = max.max(p1.area(*p2));
        }
        rectangles_processed += 1;
        let total_time = started_at.elapsed();
        let avg_time = total_time / rectangles_processed as u32;
        let rem_time = avg_time.mul_f32((total_rectangles - rectangles_processed) as f32);
        let percentage = rectangles_processed as f32 / total_rectangles as f32 * 100.0;
        if rectangles_processed % 1_000 == 0 {
            println!(
                "{} | {}/{} | {:.2}% | total: {:?} | avg: {:?} | rem: {:?}",
                max,
                rectangles_processed,
                total_rectangles,
                percentage,
                total_time,
                avg_time,
                rem_time
            );
        }
    });
    Some(max)
}

use crate::{
    common::grid::{Grid, Point},
    year_2025::day_09::{
        boundary::Boundary, for_each_combination::ForEachCombination, rectangle::Rectangle,
    },
};
use std::collections::HashSet;

pub fn part_02(input: &[Point]) -> Option<usize> {
    let boundary: HashSet<Point> = (0..input.len())
        .filter_map(|p| {
            let p1 = input.get(p)?;
            let p2 = input.get((p + 1) % input.len())?;
            p1.axis_line_to(*p2)
        })
        .flatten()
        .collect();
    // ============debug============
    let mut grid = Grid::from_points_as_bounds_with_default(
        boundary.clone().into_iter().collect::<Vec<Point>>(),
        '.',
    )
    .unwrap();
    for point in boundary.iter() {
        grid.set_value_at_point('x', *point);
    }
    let mut output = std::fs::File::create(
        "/Users/scottyrayfermo/Documents/education/rust/rusty-aoc/src/lib/year_2025/day_09/output.txt",
    ).unwrap();
    grid.write_to(&mut output).unwrap();
    // ============debug============

    let in_bound_ranges = boundary.in_bounds_ranges()?;
    let mut max = 0;
    input.for_each_combination(|p1, p2| {
        if p1
            .ranges_of_rectangle_with_other_corner(*p2)
            .iter()
            .all(|(row, rect_range)| {
                let Some(row_of_ranges) = in_bound_ranges.get(&row) else {
                    return false;
                };
                row_of_ranges.iter().any(|bound_range| {
                    bound_range.contains(rect_range.start())
                        && bound_range.contains(rect_range.end())
                })
            })
        {
            max = max.max(p1.area_of_rectangle_with_other_corner(*p2));
        }
    });
    Some(max)
}

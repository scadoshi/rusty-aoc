use crate::{
    common::grid::Point,
    year_2025::day_09::{
        for_each_combination::ForEachCombination, in_bounds_area::InBoundsAreaFromVertices,
        rectangle::OfRectangleWithOtherCorner,
    },
};

pub fn part_02(input: &[Point]) -> Option<usize> {
    let in_bounds_area = input.in_bounds_area_from_vertices();
    let mut max = 0;
    input.for_each_combination(|p1, p2| {
        let Some(area) = p1.area(*p2) else {
            return;
        };
        if area <= max {
            return;
        };
        if p1.perimeter(*p2).iter().all(|(row, perimeter_ranges)| {
            let Some(boundary_ranges) = in_bounds_area.get(row) else {
                return false;
            };
            perimeter_ranges.iter().all(|pr| {
                boundary_ranges
                    .iter()
                    .any(|br| br.contains(pr.start()) && br.contains(pr.end()))
            })
        }) {
            max = max.max(area);
        }
    });
    Some(max)
}

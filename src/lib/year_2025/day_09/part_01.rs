use crate::{common::grid::Point, year_2025::day_09::rectangle::Rectangle};

pub fn part_01(input: &[Point]) -> usize {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            input
                .iter()
                .take(i + 1)
                .map(|p2| p1.area_of_rectangle_with_other_corner(*p2))
        })
        .max()
        .unwrap()
}

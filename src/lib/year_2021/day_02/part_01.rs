use crate::year_2021::day_02::{movement::Movement, point::Point};

pub fn part_01(input: &[Movement]) -> i32 {
    input
        .iter()
        .fold(Point::new(), |mut point, movement| {
            point.traverse(movement);
            point
        })
        .product()
        .abs()
}

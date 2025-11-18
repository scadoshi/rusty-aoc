use crate::year_2021::day_02::{movement::Movement, point::AimedPoint};

pub fn part_02(input: &[Movement]) -> i32 {
    input
        .iter()
        .fold(AimedPoint::new(), |mut point, movement| {
            point.adjust(movement);
            point
        })
        .point
        .product()
        .abs()
}

use crate::year_2016::day_01::models::{DirectionalPoint, Instruction};

#[allow(dead_code)]
pub fn part_01(input: &[Instruction]) -> i32 {
    input
        .iter()
        .fold(
            DirectionalPoint::default(),
            |mut directional_point: DirectionalPoint, instruction| {
                directional_point.turn(&instruction.direction);
                directional_point.travel(instruction.distance);
                directional_point
            },
        )
        .point
        .distance_from_origin()
}

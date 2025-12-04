use crate::year_2016::day_01::{directional_point::DirectionalPoint, instruction::Instruction};

#[allow(dead_code)]
pub fn part_01(input: &[Instruction]) -> usize {
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

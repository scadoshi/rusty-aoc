use crate::year_2016::day_01::models::{DirectionalPoint, Instruction, Point};
use std::collections::HashSet;

#[allow(dead_code)]
pub fn part_02(input: &[Instruction]) -> Option<i32> {
    let mut directional_point = DirectionalPoint::default();
    let mut visited: HashSet<Point> = HashSet::from([Point::default()]);
    for instruction in input.iter() {
        directional_point.turn(&instruction.direction);
        for _ in 0..instruction.distance {
            directional_point.step();
            if visited.contains(&directional_point.point) {
                return Some(directional_point.point.distance_from_origin());
            } else {
                visited.insert(directional_point.point.clone());
            }
        }
    }
    None
}

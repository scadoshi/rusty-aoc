use crate::year_2019::day_03::{
    instruction::{ToWire, WireInstruction},
    point::Point,
};
use std::collections::HashSet;

pub fn part_01(input: &(WireInstruction, WireInstruction)) -> usize {
    let (wire1, wire2) = (input.0.to_wire(), input.1.to_wire());
    let points1: HashSet<Point> = wire1.into_iter().collect();
    wire2
        .into_iter()
        .filter(|point| points1.contains(point) && *point != Point::new())
        .map(|point| point.distance_from_origin())
        .min()
        .unwrap()
}

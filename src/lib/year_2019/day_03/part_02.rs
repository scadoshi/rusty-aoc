use crate::{
    common::cartesian_point::Point,
    year_2019::day_03::instruction::{ToWire, WireInstruction},
};
use std::collections::HashMap;

pub fn part_02(input: &(WireInstruction, WireInstruction)) -> usize {
    let (wire1, wire2) = (input.0.to_wire_with_steps(), input.1.to_wire_with_steps());
    let wire1_map: HashMap<Point, usize> = wire1.iter().map(|p| (p.point, p.steps)).collect();
    let wire2_map: HashMap<Point, usize> = wire2.iter().map(|p| (p.point, p.steps)).collect();
    wire1_map
        .keys()
        .filter(|point| wire2_map.contains_key(point) && **point != Point::new())
        .map(|point| wire1_map[point] + wire2_map[point])
        .min()
        .unwrap()
}

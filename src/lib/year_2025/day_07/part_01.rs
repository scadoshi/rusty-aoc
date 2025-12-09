use crate::common::{
    direction::Direction,
    grid::{Grid, Point},
};
use std::collections::HashMap;

pub fn part_01(input: &Grid<&'static u8>) -> usize {
    let mut total = 0;
    let start = input.find_point_with_value(&b'S').unwrap();
    let mut points = HashMap::<Point, usize>::from([(start, 1)]);
    while !points.is_empty() {
        let mut next = HashMap::<Point, usize>::new();
        points.iter().for_each(|(point, count)| {
            if let Some(down) = point.next_point_in_direction(Direction::Down)
                && let Some(value) = input.get_value_at_point(down)
            {
                match value {
                    &b'^' => {
                        total += 1;
                        [
                            down.next_point_in_direction(Direction::Left),
                            down.next_point_in_direction(Direction::Right),
                        ]
                        .into_iter()
                        .flatten()
                        .for_each(|point| *next.entry(point).or_default() += count)
                    }
                    _ => *next.entry(down).or_default() += count,
                }
            }
        });
        points = next;
    }
    total
}

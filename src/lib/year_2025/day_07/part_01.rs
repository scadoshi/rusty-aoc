use crate::common::{
    direction::Direction,
    grid::{Grid, GridOps, Point},
};
use std::collections::{HashSet, VecDeque};

pub fn part_01(input: &Grid<&'static u8>) -> usize {
    let mut queue = VecDeque::from([input.find_point_with_value(&b'S').unwrap()]);
    let mut visited = HashSet::<Point>::new();
    let mut total = 0;
    while let Some(point) = queue.pop_front() {
        if visited.contains(&point) {
            continue;
        } else {
            visited.insert(point);
        }
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
                    .for_each(|point| queue.push_front(point));
                }
                _ => {
                    queue.push_front(down);
                }
            }
        }
    }
    total
}

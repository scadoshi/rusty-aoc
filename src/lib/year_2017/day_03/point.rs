use crate::common::{cartesian_point::Point, direction::Direction};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpiralingPoint {
    pub point: Point,
    pub direction: Direction,
    pub value: usize,
    pub steps: usize,
}

impl Default for SpiralingPoint {
    fn default() -> Self {
        Self {
            point: Point::new(),
            direction: Direction::default(),
            value: 1,
            steps: 1,
        }
    }
}

impl SpiralingPoint {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&mut self) -> &mut Self {
        match self.direction {
            Direction::Right => self.point.x += 1,
            Direction::Up => self.point.y += 1,
            Direction::Left => self.point.x -= 1,
            Direction::Down => self.point.y -= 1,
        }
        self.value += 1;
        self
    }

    pub fn turn_left(&mut self) -> &mut Self {
        self.direction.turn_left();
        self
    }
    #[allow(dead_code)]
    fn is_adjacent_to(&self, other: SpiralingPoint) -> bool {
        (self.point.x - other.point.x).abs() <= 1 && (self.point.y - other.point.y).abs() <= 1
    }
    #[allow(dead_code)]
    pub fn sum_of_surrounding_point_values(&self, visited: &HashSet<SpiralingPoint>) -> usize {
        visited
            .iter()
            .filter(|other| self.is_adjacent_to(**other))
            .map(|other| other.value)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_turn_left() {
        let mut spoint = SpiralingPoint::new();
        spoint.turn_left();
        assert_eq!(spoint.direction, Direction::Left);
    }
    #[test]
    fn test_step_right() {
        let mut spoint = SpiralingPoint::new();
        spoint.turn_left().turn_left().turn_left();
        spoint.step();
        assert_eq!(spoint.point, Point { x: 1, y: 0 });
    }
    #[test]
    fn test_step_up() {
        let mut spoint = SpiralingPoint::new();
        spoint.step();
        assert_eq!(spoint.point, Point { x: 0, y: 1 });
    }
    #[test]
    fn test_step_left() {
        let mut spoint = SpiralingPoint::new();
        spoint.turn_left();
        spoint.step();
        assert_eq!(spoint.point, Point { x: -1, y: 0 });
    }
    #[test]
    fn test_step_down() {
        let mut spoint = SpiralingPoint::new();
        spoint.turn_left().turn_left();
        spoint.step();
        assert_eq!(spoint.point, Point { x: 0, y: -1 });
    }
    #[test]
    fn test_is_adjacent_to_true() {
        let spoint1 = SpiralingPoint::new();
        let mut spoint2 = spoint1.clone();
        spoint2.step();
        assert!(spoint1.is_adjacent_to(spoint2));
    }
    #[test]
    fn test_is_adjacent_to_false() {
        let spoint1 = SpiralingPoint::new();
        let spoint2 = SpiralingPoint {
            point: Point { x: 2, y: 0 },
            ..Default::default()
        };
        assert!(!spoint1.is_adjacent_to(spoint2));
    }
    #[test]
    fn test_sum_of_surrounding_point_values() {
        let visited = HashSet::from([
            SpiralingPoint {
                point: Point { x: 0, y: 0 },
                value: 1,
                ..SpiralingPoint::default()
            },
            SpiralingPoint {
                point: Point { x: 1, y: 0 },
                value: 1,
                ..SpiralingPoint::default()
            },
            SpiralingPoint {
                point: Point { x: 1, y: 1 },
                value: 1,
                ..SpiralingPoint::default()
            },
            SpiralingPoint {
                point: Point { x: 0, y: 1 },
                value: 1,
                ..SpiralingPoint::default()
            },
        ]);
        let spoint = SpiralingPoint {
            point: Point { x: -1, y: 1 },
            ..SpiralingPoint::default()
        };
        assert_eq!(2, spoint.sum_of_surrounding_point_values(&visited));
    }
}

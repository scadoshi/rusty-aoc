use std::collections::HashSet;

use crate::year_2017::day_03::direction::Direction as D;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn distance_from_origin(&self) -> usize {
        usize::try_from((self.x).abs() + (self.y).abs()).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpiralingPoint {
    pub point: Point,
    pub direction: D,
    pub value: usize,
    pub steps: usize,
}

impl Default for SpiralingPoint {
    fn default() -> Self {
        Self {
            point: Point::new(),
            direction: D::default(),
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
            D::Right => self.point.x += 1,
            D::Up => self.point.y += 1,
            D::Left => self.point.x -= 1,
            D::Down => self.point.y -= 1,
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use SpiralingPoint as Spoint;
//     #[test]
//     fn test_turn_left() {
//         let mut spoint = Spoint::new();
//         spoint.turn_left();
//         assert_eq!(spoint.direction, D::Up);
//     }
//     #[test]
//     fn test_step_right() {
//         let mut spoint = Spoint::new();
//         spoint.step();
//         assert_eq!(
//             spoint,
//             Spoint {
//                 point: Point { x: 1, y: 0 },
//                 direction: D::Right,
//                 value: 2,
//                 steps: 1,
//             }
//         );
//     }
//     #[test]
//     fn test_step_up() {
//         let mut spoint = Spoint::new();
//         spoint.turn_left();
//         spoint.step();
//         assert_eq!(
//             spoint,
//             Spoint {
//                 point: Point { x: 0, y: 1 },
//                 direction: D::Up,
//                 value: 2,
//                 steps: 1,
//             }
//         );
//     }
//     #[test]
//     fn test_step_left() {
//         let mut spoint = Spoint::new();
//         spoint.turn_left().turn_left();
//         spoint.step();
//         assert_eq!(
//             spoint,
//             Spoint {
//                 point: Point { x: -1, y: 0 },
//                 direction: D::Left,
//                 value: 2,
//                 steps: 1,
//             }
//         );
//     }
//     #[test]
//     fn test_step_down() {
//         let mut spoint = Spoint::new();
//         spoint.turn_left().turn_left().turn_left();
//         spoint.step();
//         assert_eq!(
//             spoint,
//             Spoint {
//                 point: Point { x: 0, y: -1 },
//                 direction: D::Down,
//                 value: 2,
//                 steps: 1,
//             }
//         );
//     }
//     #[test]
//     fn test_is_adjacent_to_true() {
//         let spoint1 = Spoint::new();
//         let mut spoint2 = spoint1.clone();
//         spoint2.step();
//         assert!(spoint1.is_adjacent_to(spoint2));
//     }
//     #[test]
//     fn test_is_adjacent_to_false() {
//         let spoint1 = Spoint::new();
//         let spoint2 = Spoint {
//             point: Point { x: 2, y: 0 },
//             ..Default::default()
//         };
//         assert!(!spoint1.is_adjacent_to(spoint2));
//     }
//     #[test]
//     fn test_sum_of_surrounding_point_values() {
//         let visited = HashSet::from([
//             Spoint {
//                 point: Point { x: 0, y: 0 },
//                 value: 1,
//                 ..Spoint::default()
//             },
//             Spoint {
//                 point: Point { x: 1, y: 0 },
//                 value: 1,
//                 ..Spoint::default()
//             },
//             Spoint {
//                 point: Point { x: 1, y: 1 },
//                 value: 1,
//                 ..Spoint::default()
//             },
//             Spoint {
//                 point: Point { x: 0, y: 1 },
//                 value: 1,
//                 ..Spoint::default()
//             },
//         ]);
//         let spoint = Spoint {
//             point: Point { x: -1, y: 1 },
//             ..Spoint::default()
//         };
//         assert_eq!(2, spoint.sum_of_surrounding_point_values(&visited));
//     }
// }

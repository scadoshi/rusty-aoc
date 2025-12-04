use std::collections::HashSet;

use crate::common::{
    direction::Direction,
    grid::{Grid, GridOps, Point},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Guard {
    Present { point: Point, direction: Direction },
    Away,
}

impl Guard {
    pub fn is_present(&self) -> bool {
        match self {
            Self::Present { .. } => true,
            Self::Away => false,
        }
    }

    pub fn get_point(&self) -> Option<Point> {
        match self {
            Self::Present { point, .. } => Some(*point),
            Self::Away => None,
        }
    }

    pub fn next_move(&self, grid: &Grid<char>) -> Self {
        match self {
            Self::Present { point, direction } => {
                let Some(next) = point.next_point_in_direction(*direction) else {
                    return Guard::Away;
                };
                let Some(value) = grid.get_value_at_point(next) else {
                    return Guard::Away;
                };
                if value == '#' {
                    Guard::Present {
                        direction: direction.to_right(),
                        point: *point,
                    }
                } else {
                    Guard::Present {
                        direction: *direction,
                        point: next,
                    }
                }
            }
            Self::Away => Self::Away,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    pub visited: HashSet<Point>,
    pub infinite_loop_encountered: bool,
}

pub trait GuardOps {
    fn find_guard(&self) -> Guard;
    fn simulate_guard(&self) -> Path;
    fn place_obstacle(&mut self, point: Point);
}

impl GuardOps for Grid<char> {
    fn find_guard(&self) -> Guard {
        for (point, value) in self.to_points_with_values() {
            if ['^', '>', 'v', '<'].contains(&value) {
                let direction = Direction::from(value);
                return Guard::Present { point, direction };
            }
        }
        Guard::Away
    }

    fn simulate_guard(&self) -> Path {
        let mut guard = self.find_guard();
        let mut seen = HashSet::<Guard>::new();
        let mut infinite_loop_encountered = false;
        while guard.is_present() {
            seen.insert(guard);
            guard = guard.next_move(self);
            if seen.contains(&guard) {
                infinite_loop_encountered = true;
                break;
            }
        }
        let visited: HashSet<Point> = seen.into_iter().filter_map(|g| g.get_point()).collect();
        Path {
            visited,
            infinite_loop_encountered,
        }
    }

    fn place_obstacle(&mut self, point: Point) {
        let Some(row) = self.get_mut(point.row) else {
            return;
        };
        if let Some(value) = row.get_mut(point.col) {
            *value = '#';
        }
    }
}

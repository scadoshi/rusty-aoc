use crate::year_2016::day_01::directions::{FacingDirection, TurningDirection};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn distance_from_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct DirectionalPoint {
    pub point: Point,
    pub facing_direction: FacingDirection,
}

impl DirectionalPoint {
    pub fn turn(&mut self, direction: &TurningDirection) {
        self.facing_direction.turn(direction)
    }

    pub fn travel(&mut self, distance: i32) {
        match self.facing_direction {
            FacingDirection::Left => self.point.x -= distance,
            FacingDirection::Right => self.point.x += distance,
            FacingDirection::Down => self.point.y -= distance,
            FacingDirection::Up => self.point.y += distance,
        }
    }

    pub fn step(&mut self) {
        self.travel(1);
    }
}

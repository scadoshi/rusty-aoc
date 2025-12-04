use crate::{
    common::{cartesian_point::Point, direction::Direction},
    year_2016::day_01::turn::{Turn, TurnDirection},
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct DirectionalPoint {
    pub point: Point,
    pub facing_direction: Direction,
}

impl DirectionalPoint {
    pub fn turn(&mut self, direction: &TurnDirection) {
        self.facing_direction.turn(direction)
    }

    pub fn travel(&mut self, distance: isize) {
        match self.facing_direction {
            Direction::Left => self.point.x -= distance,
            Direction::Right => self.point.x += distance,
            Direction::Down => self.point.y -= distance,
            Direction::Up => self.point.y += distance,
        }
    }

    pub fn step(&mut self) {
        self.travel(1);
    }
}

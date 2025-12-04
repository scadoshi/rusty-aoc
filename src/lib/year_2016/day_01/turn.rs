use crate::common::direction::Direction;

#[derive(Debug, Clone)]
pub enum TurnDirection {
    Left,
    Right,
}

pub trait Turn {
    fn turn(&mut self, turn_direction: &TurnDirection);
}

impl Turn for Direction {
    fn turn(&mut self, turn_direction: &TurnDirection) {
        match turn_direction {
            TurnDirection::Left => match self {
                Direction::Left => *self = Direction::Down,
                Direction::Right => *self = Direction::Up,
                Direction::Up => *self = Direction::Left,
                Direction::Down => *self = Direction::Right,
            },
            TurnDirection::Right => match self {
                Direction::Left => *self = Direction::Up,
                Direction::Right => *self = Direction::Down,
                Direction::Up => *self = Direction::Right,
                Direction::Down => *self = Direction::Left,
            },
        }
    }
}

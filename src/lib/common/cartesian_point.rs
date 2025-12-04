use crate::common::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn distance_from_origin(&self) -> usize {
        usize::try_from(self.x.abs() + self.y.abs()).unwrap()
    }

    pub fn step(&mut self, direction: Direction) -> &mut Self {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
        }
        self
    }
}

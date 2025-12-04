use crate::year_2019::day_03::direction::Direction;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new() -> Self {
        Self::default()
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

    pub fn distance_from_origin(&self) -> usize {
        usize::try_from((self.x).abs() + (self.y).abs()).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct PointWithSteps {
    pub point: Point,
    pub steps: usize,
}

impl PointWithSteps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&mut self, direction: Direction) -> &mut Self {
        self.point.step(direction);
        self.steps += 1;
        self
    }
}

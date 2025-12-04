use crate::common::{cartesian_point::Point, direction::Direction};

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

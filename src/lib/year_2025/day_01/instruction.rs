use crate::year_2025::day_01::direction::Direction;

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub direction: Direction,
    pub distance: usize,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut parts = value.chars();
        let direction = Direction::from(parts.next().unwrap());
        let distance: usize = parts.collect::<String>().parse().unwrap();
        Self {
            direction,
            distance,
        }
    }
}

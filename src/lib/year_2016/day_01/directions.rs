#[derive(Debug, Clone)]
pub enum TurningDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum FacingDirection {
    Left,
    Right,
    #[default]
    Up,
    Down,
}

impl FacingDirection {
    pub fn turn(&mut self, direction: &TurningDirection) {
        match direction {
            TurningDirection::Left => match self {
                FacingDirection::Left => *self = FacingDirection::Down,
                FacingDirection::Right => *self = FacingDirection::Up,
                FacingDirection::Up => *self = FacingDirection::Left,
                FacingDirection::Down => *self = FacingDirection::Right,
            },
            TurningDirection::Right => match self {
                FacingDirection::Left => *self = FacingDirection::Up,
                FacingDirection::Right => *self = FacingDirection::Down,
                FacingDirection::Up => *self = FacingDirection::Right,
                FacingDirection::Down => *self = FacingDirection::Left,
            },
        }
    }
}

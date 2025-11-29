#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Direction {
    #[default]
    Right,
    Down,
    Left,
    Up,
}

use Direction as D;

impl Direction {
    pub fn turn_left(&mut self) -> &mut Self {
        match self {
            D::Up => *self = D::Left,
            D::Left => *self = D::Down,
            D::Down => *self = D::Right,
            D::Right => *self = D::Up,
        }
        self
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value.to_ascii_lowercase() {
            '^' | 'u' | 'n' => Self::Up,
            '>' | 'r' | 'e' => Self::Right,
            'v' | 'd' | 's' => Self::Down,
            '<' | 'l' | 'w' => Self::Left,
            _ => Self::default(),
        }
    }
}

impl Direction {
    pub fn to_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_left(&mut self) {
        *self = self.to_left();
    }

    pub fn to_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn turn_right(&mut self) {
        *self = self.to_right();
    }
}

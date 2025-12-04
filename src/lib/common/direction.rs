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
    pub fn turn_left(&mut self) -> &mut Self {
        match self {
            Direction::Up => *self = Direction::Left,
            Direction::Left => *self = Direction::Down,
            Direction::Down => *self = Direction::Right,
            Direction::Right => *self = Direction::Up,
        }
        self
    }
}

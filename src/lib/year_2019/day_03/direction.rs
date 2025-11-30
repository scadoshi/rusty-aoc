#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value.to_ascii_lowercase() {
            'u' => Self::Up,
            'r' => Self::Right,
            'd' => Self::Down,
            'l' => Self::Left,
            x => panic!("{} is invalid direction", x),
        }
    }
}

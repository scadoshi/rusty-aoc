#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value.to_ascii_lowercase() {
            'r' => Self::Right,
            'l' => Self::Left,
            x => panic!("{} is not a invalid direction", x),
        }
    }
}

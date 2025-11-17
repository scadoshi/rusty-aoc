#[derive(Debug)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value.to_lowercase().next() {
            Some('u') => Direction::Up,
            Some('d') => Direction::Down,
            Some('l') => Direction::Left,
            Some('r') => Direction::Right,
            a => panic!("{a:?} is not a valid direction"),
        }
    }
}

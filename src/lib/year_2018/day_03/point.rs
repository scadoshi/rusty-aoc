#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        // e.g. "55,885"
        let (from_left_str, from_top_str) = value.trim().split_once(',').unwrap();
        let (row, col) = (
            from_top_str.parse::<usize>().unwrap(),
            from_left_str.parse::<usize>().unwrap(),
        );
        Point { row, col }
    }
}

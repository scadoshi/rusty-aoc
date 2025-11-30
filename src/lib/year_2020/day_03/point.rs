#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new() -> Self {
        Self::default()
    }
}

pub trait ValueAtPoint {
    fn value_at_point(&self, point: Point) -> Option<char>;
}

impl ValueAtPoint for &[Vec<char>] {
    fn value_at_point(&self, point: Point) -> Option<char> {
        let Some(row) = self.get(point.row) else {
            return None;
        };
        row.get(point.col).copied()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn adjacent_points(&self) -> Vec<Point> {
        vec![
            Point {
                row: self.row + 1,
                col: self.col,
            },
            Point {
                row: self.row + 1,
                col: self.col + 1,
            },
            Point {
                row: self.row + 1,
                col: self.col - 1,
            },
            Point {
                row: self.row - 1,
                col: self.col,
            },
            Point {
                row: self.row - 1,
                col: self.col + 1,
            },
            Point {
                row: self.row - 1,
                col: self.col - 1,
            },
            Point {
                row: self.row,
                col: self.col + 1,
            },
            Point {
                row: self.row,
                col: self.col - 1,
            },
        ]
    }
}

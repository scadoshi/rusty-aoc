#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn adjacent_points(&self) -> Vec<Point> {
        let mut points = Vec::<Point>::new();
        for dr in -1_isize..=1 {
            if let Some(row) = self.row.checked_add_signed(dr) {
                for dc in -1_isize..=1 {
                    if let Some(col) = self.col.checked_add_signed(dc) {
                        let point = Point { row, col };
                        if *self != point {
                            points.push(Point { row, col })
                        }
                    }
                }
            }
        }
        points
    }
}

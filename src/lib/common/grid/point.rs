use crate::common::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn at(row: usize, col: usize) -> Self {
        Point { row, col }
    }

    pub fn adjacent_points(&self) -> Vec<Point> {
        let mut points = Vec::<Point>::new();
        for dr in -1_isize..=1 {
            if let Some(row) = self.row.checked_add_signed(dr) {
                for dc in -1_isize..=1 {
                    if let Some(col) = self.col.checked_add_signed(dc) {
                        let point = Point::at(row, col);
                        if *self != point {
                            points.push(point)
                        }
                    }
                }
            }
        }
        points
    }

    pub fn next_point_in_direction(&self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Up => self.row.checked_sub(1).map(|row| Point::at(row, self.col)),
            Direction::Right => self.col.checked_add(1).map(|col| Point::at(self.row, col)),
            Direction::Down => self.row.checked_add(1).map(|row| Point::at(row, self.col)),
            Direction::Left => self.col.checked_sub(1).map(|col| Point::at(self.row, col)),
        }
    }

    pub fn pivoted(self) -> Self {
        Point::at(self.col, self.row)
    }

    pub fn axis_line_to(&self, other: Point) -> Option<Vec<Point>> {
        if self.row == other.row {
            let start = self.col.min(other.col);
            let end = self.col.max(other.col);
            let points: Vec<Point> = (start..=end).map(|col| Point::at(self.row, col)).collect();
            Some(points)
        } else if self.col == other.col {
            let start = self.row.min(other.row);
            let end = self.row.max(other.row);
            let points: Vec<Point> = (start..=end).map(|row| Point::at(row, self.col)).collect();
            Some(points)
        } else {
            None
        }
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
        Point::at(row, col)
    }
}

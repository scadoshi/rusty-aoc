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
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        // e.g. "55,885"
        let (col, row) = value.trim().split_once(',').unwrap();
        let (row, col) = (row.parse::<usize>().unwrap(), col.parse::<usize>().unwrap());
        Point::at(row, col)
    }
}

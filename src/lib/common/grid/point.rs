use crate::common::direction::Direction;

pub type Line: Vec<Point>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn adjacent_points(&self) -> Vec<Self> {
        let mut points = Vec::<Point>::new();
        for dr in -1_isize..=1 {
            if let Some(row) = self.row.checked_add_signed(dr) {
                for dc in -1_isize..=1 {
                    if let Some(col) = self.col.checked_add_signed(dc) {
                        let point = Point { row, col };
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
            Direction::Up => self
                .row
                .checked_sub(1)
                .map(|row| Point { row, col: self.col }),
            Direction::Right => self
                .col
                .checked_add(1)
                .map(|col| Point { row: self.row, col }),
            Direction::Down => self
                .row
                .checked_add(1)
                .map(|row| Point { row, col: self.col }),
            Direction::Left => self
                .col
                .checked_sub(1)
                .map(|col| Point { row: self.row, col }),
        }
    }

    pub fn pivoted(self) -> Self {
        Point {
            row: self.col,
            col: self.row,
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
        Point { row, col }
    }
}

pub type Points = Vec<Point>;

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
        let cols_equal = self.col == other.col;
        let rows_equal = self.row == other.row;
        if (cols_equal && rows_equal) || (!cols_equal && !rows_equal) {
            return None;
        }
        if cols_equal {
            let start = self.row.min(other.row);
            let end = self.row.max(other.row);
            Some((start..=end).map(|row| Point::at(row, self.col)).collect())
        } else {
            let start = self.col.min(other.col);
            let end = self.col.max(other.col);
            Some((start..=end).map(|col| Point::at(self.row, col)).collect())
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn horizontal_axis_line() {
        let p1 = Point::at(0, 0);
        let p2 = Point::at(0, 1);
        let result = p1.axis_line_to(p2);
        let expected = Some(vec![Point::at(0, 0), Point::at(0, 1)]);
        assert_eq!(result, expected);
    }
    #[test]
    fn vertical_axis_line() {
        let p1 = Point::at(0, 0);
        let p2 = Point::at(1, 0);
        let result = p1.axis_line_to(p2);
        let expected = Some(vec![Point::at(0, 0), Point::at(1, 0)]);
        assert_eq!(result, expected);
    }
    #[test]
    fn axis_line_equality_failure() {
        let p1 = Point::at(0, 0);
        let p2 = Point::at(0, 0);
        let result = p1.axis_line_to(p2);
        assert_eq!(result, None);
    }
    #[test]
    fn axis_line_inequality_failure() {
        let p1 = Point::at(0, 0);
        let p2 = Point::at(1, 1);
        let result = p1.axis_line_to(p2);
        assert_eq!(result, None);
    }
}

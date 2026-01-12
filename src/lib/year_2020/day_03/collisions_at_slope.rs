use crate::common::grid::{Grid, Point};

pub trait CollisionsAtSlope {
    fn collisions_at_slope<const R: usize, const C: usize>(&self) -> usize;
}

impl CollisionsAtSlope for Grid<char> {
    fn collisions_at_slope<const R: usize, const C: usize>(&self) -> usize {
        let mut point = Point::new();
        let mut total = 0;
        while point.row < self.rows.len() {
            if let Some(value) = self.get_value_at_point(point)
                && *value == '#'
            {
                total += 1;
            }
            point.row += R;
            point.col = (point.col + C) % self.first_row_len().unwrap();
        }
        total
    }
}

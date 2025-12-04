use crate::common::grid::{Grid, GridOps, Point};

pub trait CollisionsAtSlope {
    fn collisions_at_slope<const R: usize, const C: usize>(&self) -> usize;
}

impl CollisionsAtSlope for Grid<char> {
    fn collisions_at_slope<const R: usize, const C: usize>(&self) -> usize {
        let mut point = Point::new();
        let mut total = 0;
        while point.row < self.len() {
            if let Some(value) = self.get_value_at_point(point)
                && value == '#'
            {
                total += 1;
            }
            point.row += R;
            point.col = (point.col + C) % self[0].len();
        }
        total
    }
}

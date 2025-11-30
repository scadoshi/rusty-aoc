use crate::year_2020::day_03::point::{Point, ValueAtPoint};

pub trait CollisionsAtSlope {
    fn collisions_at_slope(&self, row_change_per_step: usize, col_change_per_step: usize) -> usize;
}

impl CollisionsAtSlope for &[Vec<char>] {
    fn collisions_at_slope(&self, row_change_per_step: usize, col_change_per_step: usize) -> usize {
        let mut point = Point::new();
        let mut total = 0;
        while point.row < self.len() {
            if let Some(value) = self.value_at_point(point) {
                if value == '#' {
                    total += 1;
                }
            }
            point.row += row_change_per_step;
            point.col = (point.col + col_change_per_step) % self[0].len();
        }
        total
    }
}

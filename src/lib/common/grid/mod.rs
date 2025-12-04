pub mod point;
pub use point::{Point, Points};

pub type Grid<T> = Vec<Vec<T>>;

pub trait GridOps<T> {
    fn to_points(&self) -> Points;
    fn get_value_at_point(&self, point: Point) -> Option<T>;
}

impl<T: Clone> GridOps<T> for Grid<T> {
    fn to_points(&self) -> Vec<Point> {
        (0..self.len())
            .flat_map(|row| {
                (0..self.get(row).unwrap().len())
                    .map(|col| Point { row, col })
                    .collect::<Vec<Point>>()
            })
            .collect()
    }

    fn get_value_at_point(&self, point: Point) -> Option<T> {
        let Some(row) = self.get(point.row) else {
            return None;
        };
        row.get(point.col).cloned()
    }
}

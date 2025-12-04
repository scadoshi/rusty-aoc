pub mod point;
pub use point::{Point, Points};

pub type Grid<T> = Vec<Vec<T>>;

pub trait GridOps<T> {
    fn to_points(&self) -> Points;
    fn to_points_with_values(&self) -> Vec<(Point, T)>;
    fn get_value_at_point(&self, point: Point) -> Option<T>;
    fn set_value_at_point(&mut self, value: T, point: Point) -> bool;
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

    fn to_points_with_values(&self) -> Vec<(Point, T)> {
        (0..self.len())
            .flat_map(|row| {
                (0..self.get(row).unwrap().len())
                    .map(|col| {
                        let value = self.get(row).unwrap().get(col).unwrap();
                        (Point { row, col }, value.clone())
                    })
                    .collect::<Vec<(Point, T)>>()
            })
            .collect()
    }

    fn get_value_at_point(&self, point: Point) -> Option<T> {
        let Some(row) = self.get(point.row) else {
            return None;
        };
        row.get(point.col).cloned()
    }

    fn set_value_at_point(&mut self, value: T, point: Point) -> bool {
        let Some(row) = self.get_mut(point.row) else {
            return false;
        };
        if let Some(cell) = row.get_mut(point.col) {
            *cell = value;
            return true;
        }
        false
    }
}

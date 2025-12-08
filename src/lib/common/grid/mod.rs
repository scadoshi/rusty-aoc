pub mod point;
use std::fmt::Debug;

pub use point::{Point, Points};

pub type Grid<T> = Vec<Vec<T>>;

pub trait GridOps<T> {
    fn to_points(&self) -> Points;
    fn to_points_with_values(&self) -> Vec<(Point, T)>;
    fn get_value_at_point(&self, point: Point) -> Option<T>;
    fn set_value_at_point(&mut self, value: T, point: Point) -> bool;
    fn find_point_with_value(&self, value: T) -> Option<Point>;
    fn print(&self);
    fn clear_and_print(&self);
}

impl<T> GridOps<T> for Grid<T>
where
    T: Clone + PartialEq + Debug,
{
    fn to_points(&self) -> Points {
        (0..self.len())
            .flat_map(|row| {
                (0..self.get(row).unwrap().len())
                    .map(|col| Point { row, col })
                    .collect::<Points>()
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
        let row = self.get(point.row)?;
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

    fn find_point_with_value(&self, value: T) -> Option<Point> {
        self.to_points()
            .into_iter()
            .find(|&point| self.get_value_at_point(point).is_some_and(|v| v == value))
    }
}

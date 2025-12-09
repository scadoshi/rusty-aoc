use crate::common::grid::{Grid, Point};

pub trait Removeable {
    fn removeable(&self) -> Vec<Point>;
}
impl Removeable for Grid<char> {
    fn removeable(&self) -> Vec<Point> {
        self.to_points_with_values()
            .into_iter()
            .filter(|(point, value)| {
                *value == '@'
                    && point
                        .adjacent_points()
                        .into_iter()
                        .filter_map(|p| self.get_value_at_point(p))
                        .filter(|c| **c == '@')
                        .count()
                        < 4
            })
            .map(|(p, _)| p)
            .collect()
    }
}

use crate::year_2024::day_04::point::Point;

pub type Grid = Vec<Vec<char>>;

pub trait GridOperations {
    fn to_points(&self) -> Vec<Point>;
    fn value_at_point(&self, point: Point) -> Option<char>;
}

impl GridOperations for Grid {
    fn to_points(&self) -> Vec<Point> {
        (0..self.len())
            .map(|row| {
                (0..self[row].len())
                    .map(|col| Point { row, col })
                    .collect::<Vec<Point>>()
            })
            .flatten()
            .collect()
    }

    fn value_at_point(&self, point: Point) -> Option<char> {
        let Some(row) = self.get(point.row) else {
            return None;
        };
        row.get(point.col).cloned()
    }
}

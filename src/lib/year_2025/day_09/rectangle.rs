use crate::common::grid::{Point, Points};

pub trait Rectangle {
    fn area_of_rectangle_with_other_corner(&self, other: Point) -> usize;
    fn points_of_rectangle_with_other_corner(&self, other: Point) -> Points;
}

impl Rectangle for Point {
    fn area_of_rectangle_with_other_corner(&self, other: Point) -> usize {
        self.row.abs_diff(other.row).checked_add(1).unwrap()
            * self.col.abs_diff(other.col).checked_add(1).unwrap()
    }

    fn points_of_rectangle_with_other_corner(&self, other: Point) -> Points {
        let row_start = self.row.min(other.row);
        let col_start = self.col.min(other.col);
        let row_end = self.row.max(other.row);
        let col_end = self.col.max(other.col);
        (row_start..=row_end)
            .flat_map(|row| (col_start..=col_end).map(move |col| Point::at(row, col)))
            .collect()
    }
}

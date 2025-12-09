use crate::common::grid::Point;

pub trait Rectangle {
    fn area_of_rectangle_with_other_corner(&self, other: Point) -> usize;
}

impl Rectangle for Point {
    fn area_of_rectangle_with_other_corner(&self, other: Point) -> usize {
        self.row.abs_diff(other.row).checked_add(1).unwrap()
            * self.col.abs_diff(other.col).checked_add(1).unwrap()
    }
}

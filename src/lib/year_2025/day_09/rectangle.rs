use std::{collections::HashMap, ops::RangeInclusive};

use crate::common::grid::Point;

pub trait Rectangle {
    fn area_of_rectangle_with_other_corner(&self, other: Point) -> usize;
    fn ranges_of_rectangle_with_other_corner(
        &self,
        other: Point,
    ) -> HashMap<usize, RangeInclusive<usize>>;
}

impl Rectangle for Point {
    fn area_of_rectangle_with_other_corner(&self, other: Point) -> usize {
        self.row.abs_diff(other.row).checked_add(1).unwrap()
            * self.col.abs_diff(other.col).checked_add(1).unwrap()
    }

    fn ranges_of_rectangle_with_other_corner(
        &self,
        other: Point,
    ) -> HashMap<usize, RangeInclusive<usize>> {
        let min_row = self.row.min(other.row);
        let max_row = self.row.max(other.row);
        let min_col = self.col.min(other.col);
        let max_col = self.col.max(other.col);
        (min_row..=max_row)
            .map(|row| (row, min_col..=max_col))
            .collect()
    }
}

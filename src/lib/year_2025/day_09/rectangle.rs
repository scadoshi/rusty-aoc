use crate::common::grid::Point;
use std::{collections::HashMap, ops::RangeInclusive};

pub trait OfRectangleWithOtherCorner {
    fn height(&self, other: Point) -> usize;
    fn width(&self, other: Point) -> usize;
    fn area(&self, other: Point) -> usize;
    fn perimeter_ranges(&self, other: Point) -> HashMap<usize, RangeInclusive<usize>>;
}

impl OfRectangleWithOtherCorner for Point {
    fn height(&self, other: Point) -> usize {
        self.row.abs_diff(other.row)
    }

    fn width(&self, other: Point) -> usize {
        self.col.abs_diff(other.col)
    }

    fn area(&self, other: Point) -> usize {
        self.height(other) * self.width(other)
    }

    fn perimeter_ranges(&self, other: Point) -> HashMap<usize, RangeInclusive<usize>> {
        let mut ranges = HashMap::<usize, RangeInclusive<usize>>::new();
        let (col_start, col_end) = (self.col.min(other.col), self.col.max(other.col));
        let (row_start, row_end) = (self.row.min(other.row), self.row.max(other.row));
        ranges.insert(self.row, col_start..=col_end);
        ranges.insert(other.row, col_start..=col_end);
        (row_start..=row_end).for_each(|row| {
            ranges.insert(row, col_start..=col_start);
            ranges.insert(row, row_start..=row_start);
        });
        ranges
    }
}

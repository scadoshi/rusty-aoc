use crate::common::grid::Point;
use std::{collections::HashMap, ops::RangeInclusive};

pub trait OfRectangleWithOtherCorner {
    fn height(&self, other: Point) -> Option<usize>;
    fn width(&self, other: Point) -> Option<usize>;
    fn area(&self, other: Point) -> Option<usize>;
    fn perimeter(&self, other: Point) -> HashMap<usize, Vec<RangeInclusive<usize>>>;
}

impl OfRectangleWithOtherCorner for Point {
    fn height(&self, other: Point) -> Option<usize> {
        self.row.abs_diff(other.row).checked_add(1)
    }

    fn width(&self, other: Point) -> Option<usize> {
        self.col.abs_diff(other.col).checked_add(1)
    }

    fn area(&self, other: Point) -> Option<usize> {
        let height = self.height(other)?;
        let width = self.width(other)?;
        height.checked_mul(width)
    }

    fn perimeter(&self, other: Point) -> HashMap<usize, Vec<RangeInclusive<usize>>> {
        let mut ranges = HashMap::<usize, Vec<RangeInclusive<usize>>>::new();
        let (row_start, row_end) = (self.row.min(other.row), self.row.max(other.row));
        let (col_start, col_end) = (self.col.min(other.col), self.col.max(other.col));
        ranges
            .entry(row_start)
            .or_default()
            .push(col_start..=col_end);
        ranges.entry(row_end).or_default().push(col_start..=col_end);
        (row_start + 1..row_end).for_each(|row| {
            ranges.entry(row).or_default().push(col_start..=col_start);
            ranges.entry(row).or_default().push(col_end..=col_end);
        });
        ranges
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    #[test]
    fn height() {
        let p1 = Point::at(5, 5);
        let p2 = Point::at(10, 10);
        assert_eq!(p1.height(p2), Some(6));
    }
    #[test]
    fn width() {
        let p1 = Point::at(5, 5);
        let p2 = Point::at(10, 10);
        assert_eq!(p1.width(p2), Some(6));
    }
    #[test]
    fn area() {
        let p1 = Point::at(5, 5);
        let p2 = Point::at(10, 10);
        assert_eq!(p1.area(p2), Some(36));
    }
    #[test]
    fn perimeter() {
        let p1 = Point::at(5, 5);
        let p2 = Point::at(10, 10);
        let expected = Vec::from([
            (5, HashSet::from([5..=10])),
            (6, HashSet::from([5..=5, 10..=10])),
            (7, HashSet::from([5..=5, 10..=10])),
            (8, HashSet::from([5..=5, 10..=10])),
            (9, HashSet::from([5..=5, 10..=10])),
            (10, HashSet::from([5..=10])),
        ]);
        let mut result: Vec<(usize, HashSet<RangeInclusive<usize>>)> = p1
            .perimeter(p2)
            .into_iter()
            .map(|(row, ranges)| {
                (
                    row,
                    ranges
                        .into_iter()
                        .collect::<HashSet<RangeInclusive<usize>>>(),
                )
            })
            .collect();
        result.sort_by_key(|(k, _)| *k);
        assert_eq!(result, expected);
    }
}

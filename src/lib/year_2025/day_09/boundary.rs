use crate::common::grid::Point;
use std::{collections::HashMap, ops::RangeInclusive};

pub trait Boundary {
    fn boundary(&self) -> HashMap<usize, Vec<RangeInclusive<usize>>>;
}

impl Boundary for &[Point] {
    fn boundary(&self) -> HashMap<usize, Vec<RangeInclusive<usize>>> {
        (0..self.len())
            .filter_map(|p| {
                let p1 = self.get(p)?;
                let p2 = self.get((p + 1) % self.len())?;
                Some((p1, p2))
            })
            .fold(
                HashMap::<usize, Vec<RangeInclusive<usize>>>::new(),
                |mut boundary, (p1, p2)| {
                    assert!(p1 != p2);
                    let (row_start, row_end) = (p1.row.min(p2.row), p1.row.max(p2.row));
                    let (col_start, col_end) = (p1.col.min(p2.col), p1.col.max(p2.col));
                    assert!((row_start == row_end) || (col_start == col_end));
                    if row_start == row_end {
                        boundary
                            .entry(row_start)
                            .or_default()
                            .push(col_start..=col_end);
                    }
                    if col_start == col_end {
                        (row_start + 1..row_end).for_each(|row| {
                            boundary.entry(row).or_default().push(col_start..=col_end);
                        })
                    }
                    boundary
                },
            )
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;
    #[test]
    fn boundary() {
        let vertices = vec![
            Point::at(5, 5),
            Point::at(5, 10),
            Point::at(10, 10),
            Point::at(10, 5),
        ];
        let expected = Vec::from([
            (5, HashSet::from([5..=10])),
            (6, HashSet::from([5..=5, 10..=10])),
            (7, HashSet::from([5..=5, 10..=10])),
            (8, HashSet::from([5..=5, 10..=10])),
            (9, HashSet::from([5..=5, 10..=10])),
            (10, HashSet::from([5..=10])),
        ]);
        let mut result: Vec<(usize, HashSet<RangeInclusive<usize>>)> = vertices
            .as_slice()
            .boundary()
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

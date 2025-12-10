use crate::common::grid::Point;
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

pub trait Boundary {
    fn in_bounds_ranges(&self) -> Option<HashMap<usize, Vec<RangeInclusive<usize>>>>;
}

impl Boundary for HashSet<Point> {
    fn in_bounds_ranges(&self) -> Option<HashMap<usize, Vec<RangeInclusive<usize>>>> {
        let (row_bound, col_bound) = (
            self.iter().map(|p| p.row).max()?,
            self.iter().map(|p| p.col).max()?,
        );
        Some((0..=row_bound).fold(
            HashMap::<usize, Vec<RangeInclusive<usize>>>::new(),
            |mut grid, row| {
                let mut jumps = 0;
                let mut ranges = Vec::<RangeInclusive<usize>>::new();
                let mut current = None::<RangeInclusive<usize>>;
                (0..=col_bound).for_each(|col| {
                    if self.contains(&Point::at(row, col)) {
                        jumps += 1;
                    }
                    let in_bounds = jumps % 2 == 1;
                    // start new in bounds range
                    if in_bounds && current.is_none() {
                        current = Some(col..=col);
                    // bound is on edge of grid
                    } else if in_bounds
                        && col == col_bound
                        && let Some(r) = &current
                    {
                        ranges.push(*r.start()..=col);
                    // about to leave bound so push and clear range
                    } else if !in_bounds && let Some(r) = &current {
                        ranges.push(*r.start()..=col);
                        current = None;
                    }
                });
                grid.insert(row, ranges);
                grid
            },
        ))
    }
}

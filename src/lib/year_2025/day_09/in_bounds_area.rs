use crate::common::{direction::Direction, grid::Point};
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

pub trait InBoundsAreaFromVertices {
    fn in_bounds_area_from_vertices(&self) -> HashMap<usize, Vec<RangeInclusive<usize>>>;
}

impl InBoundsAreaFromVertices for &[Point] {
    fn in_bounds_area_from_vertices(&self) -> HashMap<usize, Vec<RangeInclusive<usize>>> {
        let mut row_bound: usize = 0;
        let mut col_bound: usize = 0;
        let lines = (0..self.len()).fold(HashSet::<Point>::new(), |mut lines, i| {
            let p1 = self.get(i).unwrap();
            let p2 = self.get((i + 1) % self.len()).unwrap();
            row_bound = row_bound.max(p1.row).max(p2.row);
            col_bound = col_bound.max(p1.col).max(p2.col);
            if let Some(line) = p1.axis_line_to(*p2) {
                lines.extend(line);
            }
            lines
        });
        (0..=row_bound).fold(
            HashMap::<usize, Vec<RangeInclusive<usize>>>::new(),
            |mut map, row| {
                let mut boundary_jumps = 0;
                let mut ranges = Vec::<RangeInclusive<usize>>::new();
                let mut current_range = None::<RangeInclusive<usize>>;
                (0..=col_bound).for_each(|col| {
                    let current_point = Point::at(row, col);
                    let previous_point = current_point.next_point_in_direction(Direction::Left);
                    let am_on_boundary = lines.contains(&current_point);
                    let was_on_boundary = previous_point.is_some_and(|p| lines.contains(&p));
                    let am_on_final_point = col == col_bound;
                    if !was_on_boundary && am_on_boundary {
                        boundary_jumps += 1;
                    }
                    let am_in_bounds = boundary_jumps % 2 == 1 || am_on_boundary;
                    if am_in_bounds {
                        if let Some(r) = &current_range {
                            current_range = Some(*r.start()..=col);
                        } else {
                            current_range = Some(col..=col);
                        }
                    }
                    if (!am_in_bounds || am_on_final_point)
                        && let Some(r) = &current_range
                    {
                        ranges.push(r.clone());
                        current_range = None;
                    }
                });
                map.insert(row, ranges);
                map
            },
        )
    }
}

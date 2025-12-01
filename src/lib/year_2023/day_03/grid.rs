use crate::year_2023::day_03::{part_number::PartNumber, point::Point};
use std::collections::HashSet;

pub type Grid = Vec<Vec<char>>;

pub trait GridOperations {
    fn points(&self) -> Vec<Point>;
    fn get_value_at_point(&self, point: Point) -> Option<char>;
    fn get_part_number_at_point(&self, point: Point) -> Option<PartNumber>;
    fn get_gear_ratio_at_point(&self, point: Point) -> Option<usize>;
}

impl GridOperations for Grid {
    fn points(&self) -> Vec<Point> {
        (0..self.len())
            .map(|row| {
                (0..self[0].len())
                    .map(|col| Point { row, col })
                    .collect::<Vec<Point>>()
            })
            .flatten()
            .collect()
    }

    fn get_value_at_point(&self, point: Point) -> Option<char> {
        let Some(row) = self.get(point.row) else {
            return None;
        };
        row.get(point.col).copied()
    }

    fn get_part_number_at_point(&self, point: Point) -> Option<PartNumber> {
        let Some(value) = self.get_value_at_point(point) else {
            return None;
        };
        let Some(_) = value.to_digit(10) else {
            return None;
        };
        if !point.adjacent_points().iter().any(|p| {
            self.get_value_at_point(*p)
                .is_some_and(|value| value != '.' && !value.is_numeric())
        }) {
            return None;
        }
        let row = &self[point.row];
        let mut col_start = point.col;
        while col_start > 0 && row.get(col_start - 1).is_some_and(|c| c.is_numeric()) {
            col_start -= 1;
        }
        let mut col_end = point.col;
        while col_end < row.len() && row.get(col_end + 1).is_some_and(|c| c.is_numeric()) {
            col_end += 1;
        }
        let start = Point {
            row: point.row,
            col: col_start,
        };
        let value: usize = row[col_start..=col_end]
            .into_iter()
            .collect::<String>()
            .parse()
            .unwrap();
        Some(PartNumber { value, start })
    }

    fn get_gear_ratio_at_point(&self, point: Point) -> Option<usize> {
        let Some(value) = self.get_value_at_point(point) else {
            return None;
        };
        if value != '*' {
            return None;
        }
        let part_numbers: HashSet<PartNumber> = point
            .adjacent_points()
            .into_iter()
            .filter_map(|p| self.get_part_number_at_point(p))
            .collect();
        if part_numbers.len() != 2 {
            return None;
        }
        Some(part_numbers.into_iter().map(|pn| pn.value).product())
    }
}

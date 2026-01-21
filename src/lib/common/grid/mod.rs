pub mod point;
use anyhow::anyhow;
pub use point::Point;
use std::{
    fmt::{Debug, Display},
    fs::File,
    io::Write,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub rows: Vec<Vec<T>>,
}

impl<T: PartialEq> Grid<T> {
    pub fn find_point_with_value(&self, value: T) -> Option<Point> {
        self.to_points()
            .into_iter()
            .find(|&point| self.get_value_at_point(point).is_some_and(|v| *v == value))
    }
}

impl<T: Clone> Grid<T> {
    pub fn to_points_with_values(&self) -> Vec<(Point, T)> {
        (0..self.rows.len())
            .flat_map(|row| {
                (0..self.rows.get(row).unwrap().len())
                    .map(|col| {
                        let value = self.rows.get(row).unwrap().get(col).unwrap();
                        (Point { row, col }, value.clone())
                    })
                    .collect::<Vec<(Point, T)>>()
            })
            .collect()
    }

    pub fn from_bounds_with_default(row: usize, col: usize, default: T) -> Self {
        (0..=row)
            .map(|_| (0..=col).map(|_| default.clone()).collect::<Vec<T>>())
            .collect()
    }

    pub fn from_points_as_bounds_with_default(
        points: impl AsRef<[Point]>,
        default: T,
    ) -> Option<Self> {
        let (Some(row), Some(col)) = (
            points.as_ref().iter().map(|p| p.row).max(),
            points.as_ref().iter().map(|p| p.col).max(),
        ) else {
            return None;
        };
        Some(Self::from_bounds_with_default(row, col, default))
    }
}

impl<T: Display> Grid<T> {
    pub fn write_to(&self, file: &mut File) -> anyhow::Result<()> {
        let (max_row, Some(max_col)) = (self.len(), self.max_row_len()) else {
            return Err(anyhow!("grid must have rows; rows must have rows"));
        };
        let max_row_digit_count = max_row.to_string().len();
        let display = self
            .iter()
            .enumerate()
            .map(|(row, value)| {
                let current_digit_count = row.to_string().len();
                let margin = max_row_digit_count + 4 - current_digit_count;
                format!("{}{}|", row, " ".repeat(margin))
                    + value
                        .iter()
                        .map(|x| format!("  {} ", x))
                        .collect::<String>()
                        .as_str()
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(file, "{}", display)?;
        let margin = max_row_digit_count + 3;
        let label: String = format!(
            "{}{}\n",
            " ".repeat(margin + 2),
            "-".repeat(margin + 10 + max_col * 3)
        ) + " ".repeat(margin + 1).as_str()
            + (0..max_col)
                .map(|i| format!(" | {}", i))
                .collect::<String>()
                .as_str();
        write!(file, "\n{}", label)?;
        Ok(())
    }
}

impl<T: Default> Grid<T> {
    pub fn from_bounds(row: usize, col: usize) -> Self {
        (0..=row)
            .map(|_| (0..=col).map(|_| T::default()).collect::<Vec<T>>())
            .collect()
    }

    pub fn from_points_as_bounds(points: impl AsRef<[Point]>) -> Option<Self> {
        let (Some(row), Some(col)) = (
            points.as_ref().iter().map(|p| p.row).max(),
            points.as_ref().iter().map(|p| p.col).max(),
        ) else {
            return None;
        };
        Some(Self::from_bounds(row, col))
    }
}

impl<T> std::ops::Deref for Grid<T> {
    type Target = [Vec<T>];
    fn deref(&self) -> &Self::Target {
        &self.rows
    }
}

impl<T> std::ops::DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rows
    }
}

impl<T> Grid<T> {
    pub fn is_empty(&self) -> bool {
        self.rows.iter().all(|row| row.is_empty())
    }

    pub fn max_row_len(&self) -> Option<usize> {
        self.rows.iter().map(|row| row.len()).max()
    }

    pub fn first_row_len(&self) -> Option<usize> {
        self.rows.first().map(|r| r.len())
    }

    pub fn last_row_len(&self) -> Option<usize> {
        self.rows.iter().next_back().map(|r| r.len())
    }

    pub fn is_square(&self) -> bool {
        let len = self.len();
        self.rows.iter().all(|row| row.len() == len)
    }

    pub fn is_rectangular(&self) -> bool {
        self.first_row_len()
            .is_some_and(|first_row_len| self.rows.iter().all(|row| row.len() == first_row_len))
    }

    pub fn to_points(&self) -> Vec<Point> {
        (0..self.rows.len())
            .flat_map(|row| {
                (0..self.rows.get(row).unwrap().len()).map(move |col| Point::at(row, col))
            })
            .collect()
    }

    pub fn get_row(&self, index: usize) -> Option<&[T]> {
        self.rows.get(index).map(|row| row.as_slice())
    }

    pub fn get_value_at_point(&self, point: Point) -> Option<&T> {
        let row = self.rows.get(point.row)?;
        row.get(point.col)
    }

    pub fn set_value_at_point(&mut self, value: T, point: Point) -> bool {
        let Some(row) = self.rows.get_mut(point.row) else {
            return false;
        };
        if let Some(cell) = row.get_mut(point.col) {
            *cell = value;
            return true;
        }
        false
    }

    pub fn get_mut_at_point(&mut self, point: Point) -> Option<&mut T> {
        self.rows
            .get_mut(point.row)
            .and_then(|row| row.get_mut(point.col))
    }
}

impl<I, T> FromIterator<I> for Grid<T>
where
    I: IntoIterator<Item = T>,
{
    fn from_iter<O: IntoIterator<Item = I>>(iter: O) -> Self {
        Grid {
            rows: iter
                .into_iter()
                .map(|i| i.into_iter().collect::<Vec<T>>())
                .collect(),
        }
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

pub mod point;
pub use point::Point;
use std::{
    fmt::{Debug, Display},
    fs::File,
    io::Write,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub values: Vec<Vec<T>>,
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
        (0..self.values.len())
            .flat_map(|row| {
                (0..self.values.get(row).unwrap().len())
                    .map(|col| {
                        let value = self.values.get(row).unwrap().get(col).unwrap();
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

    pub fn from_points_as_bounds_with_default(points: &[Point], default: T) -> Option<Self> {
        let (Some(row), Some(col)) = (
            points.iter().map(|p| p.row).max(),
            points.iter().map(|p| p.col).max(),
        ) else {
            return None;
        };
        Some(Self::from_bounds_with_default(row, col, default))
    }
}

impl<T: Debug> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = self
            .values
            .iter()
            .map(|row| {
                row.iter()
                    .map(|value| format!("{:?}", value))
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", display);
        write!(f, "{}", display)
    }
}

impl<T: Debug> Grid<T> {
    pub fn write_to(&self, file: &mut File) -> anyhow::Result<()> {
        for row in self.iter() {
            let s = row.iter().map(|x| format!(" {:?} ", x)).collect::<String>();
            write!(file, "{}", s)?
        }
        Ok(())
    }
}

impl<T: Default> Grid<T> {
    pub fn from_bounds(row: usize, col: usize) -> Self {
        (0..=row)
            .map(|_| (0..=col).map(|_| T::default()).collect::<Vec<T>>())
            .collect()
    }

    pub fn from_points_as_bounds<'a>(points: &[Point]) -> Option<Self> {
        let (Some(row), Some(col)) = (
            points.iter().map(|p| p.row).max(),
            points.iter().map(|p| p.col).max(),
        ) else {
            return None;
        };
        Some(Self::from_bounds(row, col))
    }
}

impl<T> Grid<T> {
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.iter().all(|row| row.is_empty())
    }

    pub fn max_row_len(&self) -> Option<usize> {
        self.values.iter().map(|row| row.len()).max()
    }

    pub fn first_row_len(&self) -> Option<usize> {
        self.values.iter().next().map(|r| r.len())
    }

    pub fn last_row_len(&self) -> Option<usize> {
        self.values.iter().next_back().map(|r| r.len())
    }

    pub fn is_square(&self) -> bool {
        let len = self.len();
        self.values.iter().all(|row| row.len() == len)
    }

    pub fn is_rectangular(&self) -> bool {
        self.first_row_len()
            .is_some_and(|first_row_len| self.values.iter().all(|row| row.len() == first_row_len))
    }

    pub fn to_points(&self) -> Vec<Point> {
        (0..self.values.len())
            .flat_map(|row| {
                (0..self.values.get(row).unwrap().len()).map(move |col| Point::at(row, col))
            })
            .collect()
    }

    pub fn rows<'a>(&'a self) -> &'a [Vec<T>] {
        self.values.as_slice()
    }

    pub fn get_row<'a>(&'a self, index: usize) -> Option<&'a [T]> {
        self.values.get(index).map(|row| row.as_slice())
    }

    pub fn get_value_at_point<'a>(&'a self, point: Point) -> Option<&'a T> {
        let row = self.values.get(point.row)?;
        row.get(point.col)
    }

    pub fn set_value_at_point(&mut self, value: T, point: Point) -> bool {
        let Some(row) = self.values.get_mut(point.row) else {
            return false;
        };
        if let Some(cell) = row.get_mut(point.col) {
            *cell = value;
            return true;
        }
        false
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Vec<T>> {
        self.values.iter()
    }
}

impl<I, T> FromIterator<I> for Grid<T>
where
    I: IntoIterator<Item = T>,
{
    fn from_iter<O: IntoIterator<Item = I>>(iter: O) -> Self {
        Grid {
            values: iter
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
        self.values.into_iter()
    }
}

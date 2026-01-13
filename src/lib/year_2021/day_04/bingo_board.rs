use crate::common::grid::{Grid, Point};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum InvalidBingoBoard {
    #[error("rows must be consistent length")]
    InconsistentRows,
    #[error("grid cannot be empty")]
    EmptyGrid,
    #[error("rows cannot be empty")]
    EmptyRow,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BingoBoard(Grid<(u8, bool)>);

pub trait TryFromIterator<I, T>
where
    I: IntoIterator<Item = T>,
    Self: Sized,
{
    type Error;
    fn try_from_iter<O: IntoIterator<Item = I>>(iter: O) -> Result<Self, Self::Error>;
}

impl<I> TryFromIterator<I, (u8, bool)> for BingoBoard
where
    I: IntoIterator<Item = (u8, bool)>,
    Self: Sized,
{
    type Error = InvalidBingoBoard;
    fn try_from_iter<O: IntoIterator<Item = I>>(iter: O) -> Result<Self, Self::Error> {
        let grid: Grid<(u8, bool)> = iter
            .into_iter()
            .map(|x| x.into_iter().collect::<Vec<(u8, bool)>>())
            .collect();
        let Some(first_row) = grid.rows.first() else {
            return Err(InvalidBingoBoard::EmptyGrid);
        };
        let row_length = first_row.len();
        if row_length == 0 {
            return Err(InvalidBingoBoard::EmptyRow);
        }
        if !grid.rows.iter().all(|r| r.len() == row_length) {
            return Err(InvalidBingoBoard::InconsistentRows);
        }
        Ok(BingoBoard(grid))
    }
}

impl IntoIterator for BingoBoard {
    type Item = Vec<(u8, bool)>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.rows.into_iter()
    }
}

impl BingoBoard {
    pub fn rows(&self) -> &[Vec<(u8, bool)>] {
        self.0.rows.as_slice()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Vec<(u8, bool)>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Vec<(u8, bool)>> {
        self.0.iter_mut()
    }

    pub fn get_value_at_point(&self, point: Point) -> Option<(u8, bool)> {
        self.0.get_value_at_point(point).copied()
    }

    pub fn find_point_with_value(&self, value: u8) -> Option<Point> {
        self.0.to_points().into_iter().find(|point| {
            self.0
                .get_value_at_point(*point)
                .is_some_and(|(v, _)| *v == value)
        })
    }

    pub fn row_length(&self) -> usize {
        self.rows().first().expect("must have rows").len()
    }

    pub fn dab(&mut self, point: Point) {
        if let Some((_, marked)) = self.0.get_mut_at_point(point) {
            *marked = true;
        }
    }

    pub fn has_bingo(&self) -> bool {
        // horizontal check
        if self
            .rows()
            .iter()
            .any(|row| row.iter().all(|(_, marked)| *marked))
        {
            return true;
        }
        // vertical check
        if (0..self.row_length()).any(|i| {
            self.rows()
                .iter()
                .all(|row| row.get(i).expect("must have consistent rows").1)
        }) {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_from_iter_fail_incons_rows() {
        let raw = [
            vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, false), (0, false), (0, false), (0, false)],
            vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
        ];
        assert_eq!(
            BingoBoard::try_from_iter(raw),
            Err(InvalidBingoBoard::InconsistentRows)
        );
    }
    #[test]
    fn try_from_iter_fail_empty_grid() {
        let raw: [[(u8, bool); 0]; 0] = [];
        assert_eq!(
            BingoBoard::try_from_iter(raw),
            Err(InvalidBingoBoard::EmptyGrid)
        );
    }
    #[test]
    fn try_from_iter_fail_empty_row() {
        let raw: [[(u8, bool); 0]; 5] = [[], [], [], [], []];
        assert_eq!(
            BingoBoard::try_from_iter(raw),
            Err(InvalidBingoBoard::EmptyRow)
        );
    }
    #[test]
    fn try_from_iter_success() {
        let raw = [
            vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
        ];
        assert!(BingoBoard::try_from_iter(raw).is_ok());
    }
    #[test]
    fn has_bingo_true_horizontal() {
        let raw = [
            vec![(0, true), (0, true), (0, true), (0, true), (0, true)],
            vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
        ];
        assert!(BingoBoard::try_from_iter(raw).unwrap().has_bingo());
    }
    #[test]
    fn has_bingo_true_vertical() {
        let raw = [
            vec![(0, true), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, true), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, true), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, true), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, true), (0, false), (0, false), (0, false), (0, false)],
        ];
        assert!(BingoBoard::try_from_iter(raw).unwrap().has_bingo());
    }
    #[test]
    fn has_bingo_false() {
        let raw = [
            vec![(0, true), (0, true), (0, false), (0, false), (0, false)],
            vec![(0, false), (0, false), (0, false), (0, true), (0, false)],
            vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, false), (0, true), (0, true), (0, false), (0, false)],
            vec![(0, false), (0, false), (0, true), (0, false), (0, false)],
        ];
        assert!(!BingoBoard::try_from_iter(raw).unwrap().has_bingo());
    }
}

use crate::year_2016::day_02::direction::Direction as D;

// std keypad
// i | 0 1 2
// ---------
// 0 | 1 2 3
// 1 | 4 5 6
// 2 | 7 8 9

#[derive(Debug)]
pub struct KeypadPoint {
    col: u8,
    row: u8,
}

impl KeypadPoint {
    pub fn to_char(&self) -> char {
        match self.row {
            0 => match self.col {
                0 => '1',
                1 => '2',
                2 => '3',
                _ => panic!("col max exceeded"),
            },
            1 => match self.col {
                0 => '4',
                1 => '5',
                2 => '6',
                _ => panic!("col max exceeded"),
            },
            2 => match self.col {
                0 => '7',
                1 => '8',
                2 => '9',
                _ => panic!("col max exceeded"),
            },
            _ => panic!("row max exceeded"),
        }
    }

    pub fn traverse(&mut self, direction: &D) {
        match direction {
            D::Up => self.row = self.row.saturating_sub(1),
            D::Down => self.row = (self.row + 1).min(2),
            D::Left => self.col = self.col.saturating_sub(1),
            D::Right => self.col = (self.col + 1).min(2),
        }
    }
}

impl Default for KeypadPoint {
    fn default() -> Self {
        Self { col: 1, row: 1 }
    }
}

use std::sync::LazyLock;

// weird keypad

// i | 0 1 2 3 4
// --------------
// 0 |     1
// 1 |   2 3 4
// 2 | 5 6 7 8 9
// 3 |   A B C
// 4 |     D

const WEIRD_KEYPAD: LazyLock<[[char; 5]; 5]> = LazyLock::new(|| {
    [
        ['_', '_', '1', '_', '_'],
        ['_', '2', '3', '4', '_'],
        ['5', '6', '7', '8', '9'],
        ['_', 'A', 'B', 'C', '_'],
        ['_', '_', 'D', '_', '_'],
    ]
});

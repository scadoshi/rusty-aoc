use crate::year_2016::day_02::direction::Direction as D;

// std keypad
// i | 0 1 2
// ---------
// 0 | 1 2 3
// 1 | 4 5 6
// 2 | 7 8 9

const STANDARD_KEYPAD: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

// weird keypad

// i | 0 1 2 3 4
// --------------
// 0 |     1
// 1 |   2 3 4
// 2 | 5 6 7 8 9
// 3 |   A B C
// 4 |     D

const WEIRD_KEYPAD: [[char; 5]; 5] = [
    ['_', '_', '1', '_', '_'],
    ['_', '2', '3', '4', '_'],
    ['5', '6', '7', '8', '9'],
    ['_', 'A', 'B', 'C', '_'],
    ['_', '_', 'D', '_', '_'],
];

#[derive(Debug, Clone)]
pub struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug)]
pub struct Keypad {
    point: Point,
    keypad: Vec<Vec<char>>,
}

impl Keypad {
    pub fn standard() -> Self {
        let keypad: Vec<Vec<char>> = STANDARD_KEYPAD.iter().map(|row| row.to_vec()).collect();
        let point = Point::new(1, 1);
        Self { keypad, point }
    }

    pub fn weird() -> Self {
        let keypad: Vec<Vec<char>> = WEIRD_KEYPAD.iter().map(|row| row.to_vec()).collect();
        let point = Point::new(2, 0);
        Self { keypad, point }
    }

    pub fn traverse(&mut self, direction: &D) {
        let new = self.point.traverse(direction);
        let Some(row): Option<&Vec<char>> = self.keypad.get(new.row) else {
            return;
        };
        let Some(char) = row.get(new.col) else {
            return;
        };
        if *char == '_' {
            return;
        }
        self.point = new;
    }

    pub fn char(&self) -> char {
        self.keypad[self.point.row][self.point.col]
    }
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn traverse(&self, direction: &D) -> Point {
        match direction {
            D::Up => Point::new(self.row.saturating_sub(1), self.col),
            D::Down => Point::new(self.row.saturating_add(1), self.col),
            D::Left => Point::new(self.row, self.col.saturating_sub(1)),
            D::Right => Point::new(self.row, self.col.saturating_add(1)),
        }
    }
}

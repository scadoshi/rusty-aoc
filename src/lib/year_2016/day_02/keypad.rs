use crate::year_2016::day_02::direction::Direction as D;

// std keypad
// i | 0 1 2
// ---------
// 0 | 1 2 3
// 1 | 4 5 6
// 2 | 7 8 9

const STANDARD_KEYPAD: [[Option<char>; 3]; 3] = [
    [Some('1'), Some('2'), Some('3')],
    [Some('4'), Some('5'), Some('6')],
    [Some('7'), Some('8'), Some('9')],
];

// weird keypad

// i | 0 1 2 3 4
// --------------
// 0 |     1
// 1 |   2 3 4
// 2 | 5 6 7 8 9
// 3 |   A B C
// 4 |     D

const WEIRD_KEYPAD: [[Option<char>; 5]; 5] = [
    [None, None, Some('1'), None, None],
    [None, Some('2'), Some('3'), Some('4'), None],
    [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
    [None, Some('A'), Some('B'), Some('C'), None],
    [None, None, Some('D'), None, None],
];

#[derive(Debug, Clone)]
pub struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug)]
pub struct Keypad {
    point: Point,
    keypad: Vec<Vec<Option<char>>>,
}

impl Keypad {
    pub fn standard() -> Self {
        let keypad: Vec<Vec<Option<char>>> =
            STANDARD_KEYPAD.iter().map(|row| row.to_vec()).collect();
        let point = Point::new(1, 1);
        Self { keypad, point }
    }

    pub fn weird() -> Self {
        let keypad: Vec<Vec<Option<char>>> = WEIRD_KEYPAD.iter().map(|row| row.to_vec()).collect();
        let point = Point::new(2, 0);
        Self { keypad, point }
    }

    pub fn traverse(&mut self, direction: &D) {
        let new = self.point.traverse(direction);
        let Some(row): Option<&Vec<Option<char>>> = self.keypad.get(new.row) else {
            return;
        };
        let Some(Some(_)) = row.get(new.col) else {
            return;
        };
        self.point = new;
    }

    pub fn get_char(&self) -> Option<char> {
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

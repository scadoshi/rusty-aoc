use crate::year_2024::day_04::{
    grid::{Grid, GridOperations},
    point::Point,
};

mod moves {
    pub const ALL: [(isize, isize); 8] = [
        (-1, 0),  // up
        (1, 0),   // down
        (0, -1),  // left
        (0, 1),   // right
        (-1, -1), // up left
        (-1, 1),  // up right
        (1, -1),  // down left
        (1, 1),   // down right
    ];
}

pub trait XmasAtPoint {
    fn xmas_at_point(&self, point: Point) -> usize;
}

impl XmasAtPoint for Grid {
    fn xmas_at_point(&self, point: Point) -> usize {
        let Some(value) = self.value_at_point(point) else {
            return 0;
        };
        if !value.eq_ignore_ascii_case(&'x') {
            return 0;
        }
        let mut total = 0;
        for (dr, dc) in moves::ALL {
            for i in 1..4 {
                let (Some(row), Some(col)) = (
                    point.row.checked_add_signed(dr * i),
                    point.col.checked_add_signed(dc * i),
                ) else {
                    break;
                };
                let new = Point { row, col };
                let Some(value) = self.value_at_point(new) else {
                    break;
                };
                if (i == 1 && value != 'M') || (i == 2 && value != 'A') || (i == 3 && value != 'S')
                {
                    break;
                } else if i == 3 && value == 'S' {
                    total += 1;
                }
            }
        }
        total
    }
}

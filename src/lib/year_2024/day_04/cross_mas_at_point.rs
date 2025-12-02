use crate::year_2024::day_04::{
    grid::{Grid, GridOperations},
    point::Point,
};

mod moves {
    pub const FORWARD_SLASH: ((isize, isize), (isize, isize)) = (
        (-1, 1), // up right
        (1, -1), // down left
    );

    pub const BACK_SLASH: ((isize, isize), (isize, isize)) = (
        (-1, -1), // up left
        (1, 1),   // down right
    );
}

pub trait CrossMasAtPoint {
    fn cross_mas_at_point(&self, point: Point) -> bool;
}

impl CrossMasAtPoint for Grid {
    fn cross_mas_at_point(&self, point: Point) -> bool {
        let Some(value) = self.value_at_point(point) else {
            return false;
        };
        if value.to_ascii_lowercase() != 'a' {
            return false;
        }
        for ((dr1, dc1), (dr2, dc2)) in [moves::FORWARD_SLASH, moves::BACK_SLASH] {
            let ((Some(row1), Some(col1)), (Some(row2), Some(col2))) = (
                (
                    point.row.checked_add_signed(dr1),
                    point.col.checked_add_signed(dc1),
                ),
                (
                    point.row.checked_add_signed(dr2),
                    point.col.checked_add_signed(dc2),
                ),
            ) else {
                return false;
            };
            let (new1, new2) = (
                Point {
                    row: row1,
                    col: col1,
                },
                Point {
                    row: row2,
                    col: col2,
                },
            );
            let (Some(value1), Some(value2)) =
                (self.value_at_point(new1), self.value_at_point(new2))
            else {
                return false;
            };
            if !(value1 == 'M' && value2 == 'S') && !(value1 == 'S' && value2 == 'M') {
                return false;
            }
        }
        true
    }
}

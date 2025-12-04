use crate::year_2018::day_03::{dimensions::Dimensions, point::Point};

#[derive(Debug, Clone, Copy)]
pub struct Claim {
    pub id: usize,
    pub top_left_corner: Point,
    pub dimensions: Dimensions,
}

impl Claim {
    pub fn to_points(&self) -> Vec<Point> {
        (0..self.dimensions.height)
            .flat_map(|row| {
                (0..self.dimensions.width)
                    .map(|col| Point {
                        row: row + self.top_left_corner.row,
                        col: col + self.top_left_corner.col,
                    })
                    .collect::<Vec<Point>>()
            })
            .collect()
    }

    pub fn overlaps_with(&self, other: Claim) -> bool {
        let points = self.to_points();
        let others = other.to_points();
        points.iter().any(|point| others.contains(point))
    }
}

impl From<&str> for Claim {
    fn from(value: &str) -> Self {
        // e.g. #1 @ 55,885: 22x10
        let mut parts = value.split_whitespace();
        let id: usize = parts.next().unwrap().replace("#", "").parse().unwrap();
        let _ = parts.next();
        let top_left_corner = Point::from(parts.next().unwrap().replace(":", "").as_str());
        let dimensions = Dimensions::from(parts.next().unwrap());
        Self {
            id,
            top_left_corner,
            dimensions,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn to_points_test() {
//         let claim = Claim {
//             id: 1,
//             top_left_corner: Point { row: 3, col: 1 },
//             dimensions: Dimensions {
//                 height: 3,
//                 width: 3,
//             },
//         };
//         let points = vec![
//             Point { row: 3, col: 1 },
//             Point { row: 3, col: 2 },
//             Point { row: 3, col: 3 },
//             Point { row: 4, col: 1 },
//             Point { row: 4, col: 2 },
//             Point { row: 4, col: 3 },
//             Point { row: 5, col: 1 },
//             Point { row: 5, col: 2 },
//             Point { row: 5, col: 3 },
//         ];
//         assert_eq!(claim.to_points(), points);
//     }
//     #[test]
//     fn overlaps_with_test_true() {
//         let claim1 = Claim {
//             id: 1,
//             top_left_corner: Point { row: 3, col: 1 },
//             dimensions: Dimensions {
//                 height: 4,
//                 width: 4,
//             },
//         };
//         let claim2 = Claim {
//             id: 2,
//             top_left_corner: Point { row: 1, col: 3 },
//             dimensions: Dimensions {
//                 height: 4,
//                 width: 4,
//             },
//         };
//         assert!(claim1.overlaps_with(claim2));
//     }
//     #[test]
//     fn overlaps_with_test_false() {
//         let claim1 = Claim {
//             id: 1,
//             top_left_corner: Point { row: 3, col: 1 },
//             dimensions: Dimensions {
//                 height: 4,
//                 width: 4,
//             },
//         };
//         let claim2 = Claim {
//             id: 2,
//             top_left_corner: Point { row: 5, col: 5 },
//             dimensions: Dimensions {
//                 height: 2,
//                 width: 2,
//             },
//         };
//         assert!(!claim1.overlaps_with(claim2));
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Point {
    pub fn distance_to(&self, other: Point) -> f64 {
        let (x1, y1, z1) = (f64::from(self.x), f64::from(self.y), f64::from(self.z));
        let (x2, y2, z2) = (f64::from(other.x), f64::from(other.y), f64::from(other.z));
        ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt()
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        // 322,817,812
        let mut dimensions = value.split(',').map(|x| x.parse::<u32>().unwrap());
        Self {
            x: dimensions.next().unwrap(),
            y: dimensions.next().unwrap(),
            z: dimensions.next().unwrap(),
        }
    }
}

/// given points this creates a vec of every combination and sorts from closest from each other to furthest
pub trait DistanceSortedPointCombinations {
    fn distance_sorted_point_combinations(&self) -> Vec<(Point, Point)>;
}

impl DistanceSortedPointCombinations for &[Point] {
    fn distance_sorted_point_combinations(&self) -> Vec<(Point, Point)> {
        let mut distances: Vec<(f64, Point, Point)> = self
            .iter()
            .enumerate()
            .flat_map(|(i, p1)| {
                self.iter()
                    .skip(i + 1)
                    .map(|p2| (p1.distance_to(*p2), *p1, *p2))
            })
            .collect();
        distances.sort_by(|(a, ..), (b, ..)| b.partial_cmp(a).unwrap());
        distances.iter().map(|(_, p1, p2)| (*p1, *p2)).collect()
    }
}

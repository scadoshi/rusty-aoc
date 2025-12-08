#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

impl Point {
    pub fn distance_to(&self, other: Point) -> f32 {
        let (x1, y1, z1) = (f32::from(self.x), f32::from(self.y), f32::from(self.z));
        let (x2, y2, z2) = (f32::from(other.x), f32::from(other.y), f32::from(other.z));
        ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt()
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        // 162,817,812
        let mut dimensions = value.split(',').map(|x| x.parse::<u16>().unwrap());
        Self {
            x: dimensions.next().unwrap(),
            y: dimensions.next().unwrap(),
            z: dimensions.next().unwrap(),
        }
    }
}

use crate::year_2016::day_03::triangle::Triangle;

pub trait Pivot {
    fn pivot(self) -> Vec<Triangle>;
}

impl Pivot for &[Triangle] {
    fn pivot(self) -> Vec<Triangle> {
        let (v1, v2, v3) = self.iter().fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut v1, mut v2, mut v3), t| {
                v1.push(t.0);
                v2.push(t.1);
                v3.push(t.2);
                (v1, v2, v3)
            },
        );
        let mut result: Vec<Triangle> = v1.chunks(3).map(|c| Triangle(c[0], c[1], c[2])).collect();
        result.extend(v2.chunks(3).map(|c| Triangle(c[0], c[1], c[2])));
        result.extend(v3.chunks(3).map(|c| Triangle(c[0], c[1], c[2])));
        result
    }
}

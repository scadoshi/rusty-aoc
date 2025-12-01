pub trait Gamma {
    fn gamma(&self) -> u16;
}

impl Gamma for &[u16] {
    fn gamma(&self) -> u16 {
        let mut result = 0;
        for i in 0..12 {
            let ones = self.iter().filter(|&num| num & (1 << i) != 0).count();
            if ones >= self.len() / 2 {
                result |= 1 << i;
            }
        }
        result
    }
}

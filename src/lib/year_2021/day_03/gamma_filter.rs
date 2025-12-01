pub trait GammaFilter {
    fn gamma_filter(&self) -> Option<u16>;
}

impl GammaFilter for &[u16] {
    fn gamma_filter(&self) -> Option<u16> {
        let mut nums = self.to_vec();
        for i in (0..12).rev() {
            let set_count = nums.iter().filter(|&num| num & (1 << i) != 0).count();
            let majority = set_count * 2 >= nums.len();
            nums.retain(|num| {
                let is_set = num & (1 << i) != 0;
                is_set == majority
            });
            if nums.len() == 1 {
                return Some(nums.into_iter().next().unwrap());
            }
        }
        None
    }
}

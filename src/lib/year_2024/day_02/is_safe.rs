pub trait IsSafe {
    fn is_safe(&self) -> bool;
}

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.

impl IsSafe for Vec<i32> {
    fn is_safe(&self) -> bool {
        let no_big_jumps = self.windows(2).all(|w| {
            let difference = (w[0] - w[1]).abs();
            difference > 0 && difference < 4
        });
        let all_increase = self.windows(2).all(|w| w[0] < w[1]);
        let all_decrease = self.windows(2).all(|w| w[0] > w[1]);

        no_big_jumps && (all_increase || all_decrease)
    }
}

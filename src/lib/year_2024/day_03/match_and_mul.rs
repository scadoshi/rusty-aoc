use crate::year_2024::day_03::MUL_PATTERN;

pub trait MatchAndMul {
    fn match_and_mul(&self) -> Option<usize>;
}

impl<T: AsRef<str>> MatchAndMul for T {
    fn match_and_mul(&self) -> Option<usize> {
        let Some(captures) = MUL_PATTERN.captures(self.as_ref()) else {
            return None;
        };
        let (Ok(num1), Ok(num2)) = (captures[1].parse::<usize>(), captures[2].parse::<usize>())
        else {
            return None;
        };
        Some(num1 * num2)
    }
}

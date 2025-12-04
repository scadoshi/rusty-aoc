use crate::year_2021::day_03::{epsilon::Epsilon, gamma::Gamma};

pub fn part_01(input: &[u16]) -> usize {
    usize::from(input.gamma()) * usize::from(input.epsilon())
}

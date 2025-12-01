use crate::year_2021::day_03::{epsilon::Epsilon, gamma::Gamma};

pub fn part_01(input: &[u16]) -> usize {
    input.gamma() as usize * input.epsilon() as usize
}

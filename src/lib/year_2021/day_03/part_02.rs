use crate::year_2021::day_03::{epsilon_filter::EpsilonFilter, gamma_filter::GammaFilter};

pub fn part_02(input: &[u16]) -> Option<usize> {
    let (Some(gamma), Some(epsilon)) = (input.gamma_filter(), input.epsilon_filter()) else {
        return None;
    };
    Some(usize::from(gamma) * usize::from(epsilon))
}

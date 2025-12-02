use crate::year_2024::day_03::{match_and_mul::MatchAndMul, MUL_PATTERN_LEN};

pub fn part_01(input: &[char]) -> usize {
    input
        .windows(MUL_PATTERN_LEN)
        .filter_map(|window| window.iter().collect::<String>().match_and_mul())
        .sum()
}

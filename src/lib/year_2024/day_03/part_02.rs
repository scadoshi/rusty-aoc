use crate::year_2024::day_03::{MUL_PATTERN_LEN, do_or_dont::DoOrDont, match_and_mul::MatchAndMul};

pub fn part_02(input: &[char]) -> usize {
    input
        .windows(MUL_PATTERN_LEN)
        .fold((true, 0), |(mut intaking, mut total), window| {
            let window_string: String = window.iter().collect();
            intaking = window_string.do_or_dont(intaking);
            if intaking && let Some(increment) = window_string.match_and_mul() {
                total += increment;
            }
            (intaking, total)
        })
        .1
}

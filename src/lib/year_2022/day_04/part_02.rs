use super::get_input::RangePair;

pub fn part_02(input: &[RangePair]) -> usize {
    input
        .iter()
        .filter(|((ls, le), (rs, re))| {
            (ls >= rs && ls <= re)
                || (le >= rs && le <= re)
                || (rs >= ls && rs <= le)
                || (re >= ls && re <= le)
        })
        .count()
}

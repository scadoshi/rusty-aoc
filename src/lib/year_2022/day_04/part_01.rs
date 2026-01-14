use super::get_input::RangePair;

pub fn part_01(input: &[RangePair]) -> usize {
    input
        .iter()
        .filter(|((ls, le), (rs, re))| (rs <= ls && le <= re) || (ls <= rs && re <= le))
        .count()
}

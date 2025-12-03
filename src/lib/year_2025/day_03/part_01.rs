use crate::year_2025::day_03::max_joltage::MaxJoltage;

pub fn part_01(input: &[Vec<u8>]) -> usize {
    input.iter().map(|x| x.max_joltage(2)).sum()
}

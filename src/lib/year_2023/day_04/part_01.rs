use super::card::Card;

pub fn part_01(input: &[Card]) -> u32 {
    input.iter().map(|c| c.points()).sum()
}

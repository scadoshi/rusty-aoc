use crate::year_2018::day_03::claim::Claim;

pub fn part_02(input: &[Claim]) -> Option<usize> {
    for (i, claim) in input.iter().enumerate() {
        if !input
            .iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .any(|(_, other)| claim.overlaps_with(*other))
        {
            return Some(claim.id);
        }
    }
    None
}

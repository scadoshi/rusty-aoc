use crate::year_2016::day_04::room::Room;

pub fn part_01(input: &[Room]) -> u32 {
    input
        .iter()
        .filter_map(|r| if r.is_valid() { Some(r.id) } else { None })
        .sum()
}

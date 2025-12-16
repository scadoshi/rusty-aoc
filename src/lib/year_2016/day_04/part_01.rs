use crate::year_2016::day_04::room::Room;

pub fn part_01(input: &[&'static str]) -> u32 {
    input
        .iter()
        .filter_map(|x| {
            let r = Room::from(*x);
            if r.is_valid() { Some(r.id) } else { None }
        })
        .sum()
}

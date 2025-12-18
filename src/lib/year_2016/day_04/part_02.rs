use crate::year_2016::day_04::room::Room;

pub fn part_02(input: &[Room]) -> u32 {
    input
        .iter()
        .find(|r| r.decode_name() == "northpole-object-storage")
        .unwrap()
        .id
}

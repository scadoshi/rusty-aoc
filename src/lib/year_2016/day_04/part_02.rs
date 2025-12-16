use crate::year_2016::day_04::room::Room;

pub fn part_02(input: &[&'static str]) -> u32 {
    input
        .iter()
        .map(|x| Room::from(*x))
        .find(|x| x.name == "northpole-object-storage")
        .unwrap()
        .id
}

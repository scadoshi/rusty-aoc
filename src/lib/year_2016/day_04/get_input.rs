use crate::year_2016::day_04::room::Room;

pub fn get_input() -> Vec<Room> {
    include_str!("input.txt").lines().map(Room::from).collect()
}

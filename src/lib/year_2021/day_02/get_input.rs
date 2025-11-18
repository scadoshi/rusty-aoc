use crate::year_2021::day_02::movement::Movement;

pub fn get_input() -> Vec<Movement> {
    include_str!("input.txt")
        .lines()
        .map(|line| Movement::from(line))
        .collect()
}

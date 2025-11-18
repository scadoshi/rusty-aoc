use crate::year_2022::day_02::game::RawGame;

pub fn get_input() -> Vec<RawGame> {
    include_str!("input.txt")
        .lines()
        .map(|x| RawGame::from(x))
        .collect()
}

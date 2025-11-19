use crate::year_2023::day_02::marble_game::Game;

pub fn get_input() -> Vec<Game> {
    include_str!("input.txt")
        .lines()
        .map(|x| Game::from(x))
        .collect()
}

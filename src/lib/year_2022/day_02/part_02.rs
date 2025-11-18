use crate::year_2022::day_02::game::RawGame;

pub fn part_02(input: &[RawGame]) -> i32 {
    input
        .iter()
        .map(|x| x.to_game_other_is_result().player_score())
        .sum()
}

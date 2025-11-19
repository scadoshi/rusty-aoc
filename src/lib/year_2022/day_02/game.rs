use crate::year_2022::day_02::rock_paper_scissors::RockPaperScissors as RPS;

#[derive(Debug, PartialEq)]
pub enum GameResult {
    Win,
    Lose,
    Draw,
}

use GameResult as GR;
impl GR {
    /// in order to <game result> against <other> we do <function result>
    fn against(&self, other: &RPS) -> RPS {
        let moves = vec![RPS::Rock, RPS::Paper, RPS::Scissors];
        moves
            .into_iter()
            .find(|m| m.against(other) == *self)
            .expect("failed to find valid move")
    }
}

impl From<char> for GR {
    fn from(value: char) -> Self {
        match value.to_ascii_lowercase() {
            'x' => GR::Lose,
            'y' => GR::Draw,
            'z' => GR::Win,
            x => panic!("{} is not a valid game result", x),
        }
    }
}

pub struct RawGame {
    opponent: RPS,
    other: char,
}

impl From<&str> for RawGame {
    fn from(value: &str) -> Self {
        let (opponent_str, other_str) = value.split_once(" ").expect("failed to split once");
        let (opponent, other) = (
            RPS::from(opponent_str.chars().next().expect("failed to get next")),
            other_str.chars().next().expect("failed to get next"),
        );
        Self { opponent, other }
    }
}

impl RawGame {
    pub fn to_game_other_is_player(&self) -> Game {
        let opponent = self.opponent.clone();
        let player = RPS::from(self.other);
        let result = player.against(&opponent);
        Game { player, result }
    }

    pub fn to_game_other_is_result(&self) -> Game {
        let opponent = self.opponent.clone();
        let result = GR::from(self.other);
        let player = result.against(&opponent);
        Game { player, result }
    }
}

pub struct Game {
    player: RPS,
    result: GR,
}

impl Game {
    pub fn player_score(&self) -> i32 {
        let game_result_score = match self.result {
            GR::Win => 6,
            GR::Draw => 3,
            GR::Lose => 0,
        };
        let move_score = match self.player {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };
        game_result_score + move_score
    }
}

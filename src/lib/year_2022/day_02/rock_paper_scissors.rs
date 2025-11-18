#[derive(Debug, Clone, PartialEq)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

use RockPaperScissors as RPS;

use crate::year_2022::day_02::game::GameResult;
impl From<char> for RockPaperScissors {
    fn from(value: char) -> Self {
        match value.to_ascii_lowercase() {
            'a' | 'x' => RPS::Rock,
            'b' | 'y' => RPS::Paper,
            'c' | 'z' => RPS::Scissors,
            x => panic!("cannot parse rock paper scissors from {}", x),
        }
    }
}

impl RPS {
    pub fn against(&self, other: &RPS) -> GameResult {
        if (*self == RPS::Rock && *other == RPS::Scissors)
            || (*self == RPS::Paper && *other == RPS::Rock)
            || (*self == RPS::Scissors && *other == RPS::Paper)
        {
            GameResult::Win
        } else if (*self == RPS::Rock && *other == RPS::Paper)
            || (*self == RPS::Paper && *other == RPS::Scissors)
            || (*self == RPS::Scissors && *other == RPS::Rock)
        {
            GameResult::Lose
        } else if self == other {
            GameResult::Draw
        } else {
            panic!(
                "what happens when self: {:?} and other: {:?} ???",
                self, other
            );
        }
    }
}

use crate::year_2023::day_02::marble_game::Game;

const MAXIMUMS: [(&str, u8); 3] = [("red", 12), ("green", 13), ("blue", 14)];

trait IsPossible {
    fn is_possible(&self) -> bool;
}

impl IsPossible for Game {
    fn is_possible(&self) -> bool {
        !self.handfuls.iter().any(|handful| {
            handful.marble_groups().iter().any(|marble_group| {
                MAXIMUMS.iter().any(|(c, m)| {
                    marble_group.color.to_lowercase() == *c && marble_group.count > *m
                })
            })
        })
    }
}

pub fn part_01(input: &[Game]) -> i32 {
    input
        .iter()
        .filter(|g| g.is_possible())
        .map(|g| g.id as i32)
        .sum()
}

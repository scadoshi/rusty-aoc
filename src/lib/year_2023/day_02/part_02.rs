use crate::year_2023::day_02::marble_game::Game;
use std::collections::HashMap;

pub fn part_02(input: &[Game]) -> i32 {
    input
        .iter()
        .map(|g| {
            g.handfuls.iter().fold(HashMap::new(), |mut map, handful| {
                handful.marble_groups().iter().for_each(|marble_group| {
                    let count: &mut u8 = map.entry(marble_group.color.clone()).or_default();
                    *count = (*count).max(marble_group.count);
                });
                map
            })
        })
        .map(|m| m.into_iter().map(|(_, v)| v as i32).product::<i32>())
        .sum()
}

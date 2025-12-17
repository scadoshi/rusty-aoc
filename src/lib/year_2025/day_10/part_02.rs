use crate::year_2025::day_10::{
    click::Click,
    u12::pack::{ITEM_COUNT_MAX, TryPack, compare::AnySlotsGreaterThan, len::GetPackLen},
};
use std::collections::HashSet;

pub fn part_02(input: &[(u16, Vec<u16>, Vec<u16>)]) -> usize {
    let len = input.len();
    input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let (_lights, buttons, target) = line;
            let mut seen = HashSet::<u128>::new();
            let mut clicks = 0;
            let target = target.iter().try_pack().unwrap();
            let initial = (target.len() as u128) << (ITEM_COUNT_MAX * 12);
            let mut states = HashSet::<u128>::from([initial]);
            'main: loop {
                clicks += 1;
                let mut added = HashSet::<u128>::new();
                // ======debug======
                println!(
                    "{}/{} completed | {:.2}% done | clicks: {} | states: {} | seen: {}",
                    i,
                    len,
                    i as f32 / len as f32 * 100.0,
                    clicks,
                    states
                        .len()
                        .to_string()
                        .chars()
                        .rev()
                        .collect::<Vec<_>>()
                        .chunks(3)
                        .map(|c| c.iter().collect::<String>())
                        .collect::<Vec<_>>()
                        .join(",")
                        .chars()
                        .rev()
                        .collect::<String>(),
                    seen.len()
                        .to_string()
                        .chars()
                        .rev()
                        .collect::<Vec<_>>()
                        .chunks(3)
                        .map(|c| c.iter().collect::<String>())
                        .collect::<Vec<_>>()
                        .join(",")
                        .chars()
                        .rev()
                        .collect::<String>()
                );
                // ======debug======
                for state in states.iter() {
                    for button in buttons {
                        let updated = state.click(*button);
                        if seen.contains(&updated) {
                            continue;
                        }
                        if updated == target {
                            break 'main;
                        }
                        if !updated.any_slots_greater_than(target) {
                            seen.insert(updated);
                            added.insert(updated);
                        }
                    }
                }
                states = added;
            }
            clicks
        })
        .sum()
}

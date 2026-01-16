use super::card::Card;
use std::collections::{HashMap, VecDeque};

pub fn part_02(input: &[Card]) -> u32 {
    let all_cards: HashMap<u32, &Card> = input.iter().map(|c| (c.id(), c)).collect();
    let mut queue: VecDeque<u32> = input.iter().map(|c| c.id()).collect();
    let mut total_cards = u32::try_from(input.len()).unwrap();
    while let Some(next_id) = queue.pop_front() {
        if let Some(new) = all_cards
            .get(&next_id)
            .unwrap()
            .calculate_ids_of_copies_won()
        {
            total_cards += new.end().checked_sub(*new.start()).unwrap() + 1;
            queue.extend(new);
        }
    }
    total_cards
}

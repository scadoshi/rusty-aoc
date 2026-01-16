use super::card::Card;

pub fn get_input() -> Vec<Card> {
    include_str!("input.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Card::try_from(line).unwrap())
        .collect()
}

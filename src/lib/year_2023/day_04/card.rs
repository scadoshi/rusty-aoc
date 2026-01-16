use std::{num::ParseIntError, ops::RangeInclusive};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidCard {
    #[error("id must be present")]
    MissingId,
    #[error("winners must be present")]
    MissingWinners,
    #[error("numbers must be present")]
    MissingNumbers,
    #[error("string in id position could not be parsed to int: {0}")]
    ParseIdError(ParseIntError),
    #[error("string in winners position cannot be parsed to int: {0}")]
    ParseWinnersError(ParseIntError),
    #[error("string in numbers position cannot be parsed to int: {0}")]
    ParseNumbersError(ParseIntError),
    #[error(
        "valid formatting looks like \"Card 1: 1 2 3 4 | 5 6 7 8\" matching the format \"Card <id_str>: <winners_str> | <numbers_str>\""
    )]
    Format,
}

#[derive(Debug)]
pub struct Card {
    id: u32,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    pub(super) fn new(
        id: u32,
        winners: impl IntoIterator<Item = u32>,
        numbers: impl IntoIterator<Item = u32>,
    ) -> Self {
        Self {
            id,
            winners: winners.into_iter().collect(),
            numbers: numbers.into_iter().collect(),
        }
    }

    pub(super) fn id(&self) -> u32 {
        self.id
    }

    pub(super) fn count_winners(&self) -> u32 {
        u32::try_from(
            self.numbers
                .iter()
                .filter(|n| self.winners.contains(n))
                .count(),
        )
        .unwrap_or(0)
    }

    pub(super) fn points(&self) -> u32 {
        let winners_count = self.count_winners();
        if winners_count == 0 {
            return 0;
        }
        2_u32.pow(winners_count.checked_sub(1).unwrap())
    }

    pub(super) fn calculate_ids_of_copies_won(&self) -> Option<RangeInclusive<u32>> {
        let winners_count = self.count_winners();
        if winners_count == 0 {
            return None;
        }
        Some(self.id + 1..=self.id + winners_count)
    }
}

impl TryFrom<&str> for Card {
    type Error = InvalidCard;
    fn try_from(value: &str) -> Result<Card, InvalidCard> {
        // e.g. "Card 1: 1 2 3 4 | 5 6 7 8"
        let (first, leftovers) = value.split_once(":").ok_or(InvalidCard::Format)?;
        let id_str_parts = first.split_whitespace();
        let id_str = id_str_parts.last().ok_or(InvalidCard::Format)?;
        let id: u32 = id_str.trim().parse().map_err(InvalidCard::ParseIdError)?;
        let (winners_str, numbers_str) = leftovers.split_once("|").ok_or(InvalidCard::Format)?;
        let winners: Vec<u32> = winners_str
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse())
            .collect::<Result<Vec<u32>, ParseIntError>>()
            .map_err(InvalidCard::ParseWinnersError)?;
        let numbers: Vec<u32> = numbers_str
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse())
            .collect::<Result<Vec<u32>, ParseIntError>>()
            .map_err(InvalidCard::ParseNumbersError)?;
        Ok(Self::new(id, winners, numbers))
    }
}

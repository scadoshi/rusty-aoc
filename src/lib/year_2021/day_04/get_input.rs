use crate::year_2021::day_04::bingo_board::InvalidBingoBoard;

use super::bingo_board::BingoBoard;
use super::bingo_board::TryFromIterator;

pub fn get_input() -> (Vec<u8>, Vec<BingoBoard>) {
    let mut parts = include_str!("input.txt").split("\n\n");
    let sequence: Vec<u8> = parts
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let boards: Vec<BingoBoard> = parts
        .filter(|board_str| !board_str.trim().is_empty())
        .map(|board_str| {
            BingoBoard::try_from_iter(board_str.split("\n").map(|line| {
                line.split_whitespace()
                    .map(|cell| (cell.trim().parse::<u8>().unwrap(), false))
            }))
        })
        .collect::<Result<Vec<BingoBoard>, InvalidBingoBoard>>()
        .unwrap();
    (sequence, boards)
}

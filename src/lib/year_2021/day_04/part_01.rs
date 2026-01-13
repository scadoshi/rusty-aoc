use super::bingo_board::BingoBoard;

pub fn part_01(input: &(Vec<u8>, Vec<BingoBoard>)) -> Option<u32> {
    let (sequence, bingo_boards) = input;
    let mut bingo_boards = bingo_boards.to_vec();
    for value in sequence.iter() {
        for bb in bingo_boards.iter_mut() {
            if let Some(point) = bb.find_point_with_value(*value) {
                bb.dab(point);
            }
            if bb.has_bingo() {
                let score = bb
                    .rows()
                    .iter()
                    .map(|row| {
                        row.iter()
                            .filter(|(_, marked)| !marked)
                            .map(|(value, _)| u32::from(*value))
                            .sum::<u32>()
                    })
                    .sum::<u32>();
                return Some(score * u32::from(*value));
            }
        }
    }
    None
}

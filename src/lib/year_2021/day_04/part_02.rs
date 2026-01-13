use super::bingo_board::BingoBoard;

pub fn part_02(input: &(Vec<u8>, Vec<BingoBoard>)) -> Option<u32> {
    let (sequence, bingo_boards) = input;
    let mut bingo_boards = bingo_boards.to_vec();
    let mut last_score: Option<u32> = None;
    for value in sequence.iter() {
        let mut i = 0;
        while i < bingo_boards.len() {
            if let Some(point) = bingo_boards[i].find_point_with_value(*value) {
                bingo_boards[i].dab(point);
            }
            if bingo_boards[i].has_bingo() {
                let score = bingo_boards[i]
                    .rows()
                    .iter()
                    .map(|row| {
                        row.iter()
                            .filter(|(_, marked)| !marked)
                            .map(|(value, _)| u32::from(*value))
                            .sum::<u32>()
                    })
                    .sum::<u32>();
                last_score = Some(score * u32::from(*value));
                bingo_boards.remove(i);
            } else {
                i += 1;
            }
        }
    }
    last_score
}

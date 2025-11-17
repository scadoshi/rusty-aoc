use crate::year_2015::day_02::gift_box::GiftBox;

pub fn part_01(input: &[GiftBox]) -> i32 {
    input.iter().map(|g| g.wrapping_paper_required()).sum()
}

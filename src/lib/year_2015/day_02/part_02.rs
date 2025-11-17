use crate::year_2015::day_02::gift_box::GiftBox;

pub fn part_02(input: &[GiftBox]) -> i32 {
    input.iter().map(|g| g.ribbon_required()).sum()
}

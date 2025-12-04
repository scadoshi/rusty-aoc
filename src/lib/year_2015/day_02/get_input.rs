use crate::year_2015::day_02::gift_box::GiftBox;

pub fn get_input() -> Vec<GiftBox> {
    include_str!("input.txt")
        .lines()
        .map(GiftBox::from)
        .collect()
}

use crate::year_2025::day_10::u12::{BIT_LEN, pack::ITEM_COUNT_MAX, pack::Pack};

pub trait GetPackLen {
    fn len(&self) -> usize;
}

impl GetPackLen for Pack {
    fn len(&self) -> usize {
        ((self >> (ITEM_COUNT_MAX * BIT_LEN)) & 0xFFF)
            .try_into()
            .expect("enforced max len 10 ensures safety")
    }
}

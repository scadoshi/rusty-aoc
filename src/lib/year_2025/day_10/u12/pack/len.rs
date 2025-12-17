use crate::year_2025::day_10::u12::{BIT_LEN, pack::ITEM_COUNT_MAX, pack::Pack};

pub trait GetPackLen {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

impl GetPackLen for Pack {
    fn len(&self) -> usize {
        ((self >> (ITEM_COUNT_MAX * BIT_LEN)) & 0xFFF)
            .try_into()
            .expect("enforced max len 10 ensures safety")
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::year_2025::day_10::u12::pack::TryPack;
    #[test]
    fn all_lens() {
        assert_eq!([0].iter().try_pack().unwrap().len(), 1);
        assert_eq!([0, 0].iter().try_pack().unwrap().len(), 2);
        assert_eq!([0, 0, 0].iter().try_pack().unwrap().len(), 3);
        assert_eq!([0, 0, 0, 0].iter().try_pack().unwrap().len(), 4);
        assert_eq!([0, 0, 0, 0, 0].iter().try_pack().unwrap().len(), 5);
        assert_eq!([0, 0, 0, 0, 0, 0].iter().try_pack().unwrap().len(), 6);
        assert_eq!([0, 0, 0, 0, 0, 0, 0].iter().try_pack().unwrap().len(), 7);
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 0].iter().try_pack().unwrap().len(), 8);
        assert_eq!(
            [0, 0, 0, 0, 0, 0, 0, 0, 0].iter().try_pack().unwrap().len(),
            9
        );
        assert_eq!(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                .iter()
                .try_pack()
                .unwrap()
                .len(),
            10
        );
    }
}

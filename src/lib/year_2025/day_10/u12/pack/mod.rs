pub mod compare;
pub mod len;
pub mod unpack;

use crate::year_2025::day_10::u12::{BIT_LEN, U12};
use std::ops::Deref;
use thiserror::Error;

pub const ITEM_COUNT_MAX: usize = 10;
pub type Pack = u128;

#[derive(Debug, Error)]
pub enum PackError {
    #[error("item {0} will not fit")]
    ItemOverflow(u16),
    #[error("len {0} will not fit")]
    Overflow(usize),
}

pub trait TryPack {
    fn try_pack(self) -> Result<u128, PackError>;
}

impl<I, U> TryPack for I
where
    I: Iterator<Item = U> + Clone + DoubleEndedIterator,
    U: Deref<Target = u16>,
{
    fn try_pack(self) -> Result<Pack, PackError> {
        let mut iter = self.into_iter().rev().enumerate();
        let mut output = 0;
        let mut len = 0;
        while let Some((i, num)) = iter.next() {
            len += 1;
            let u12_result = U12::try_from(*num);
            if u12_result.is_err() {
                return Err(PackError::ItemOverflow(*num));
            }
            if i > ITEM_COUNT_MAX - 1 {
                return Err(PackError::Overflow(i));
            }
            output = output | (Pack::from(*num) << i * BIT_LEN);
        }
        output = output | (len << ITEM_COUNT_MAX * BIT_LEN);
        Ok(output)
    }
}

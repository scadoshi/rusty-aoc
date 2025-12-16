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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::year_2025::day_10::u12::{self, pack::unpack::Unpack};
    #[test]
    fn pack_success() {
        let arr = [1, 2, 3, 4, 5];
        let packed = arr.iter().try_pack();
        assert!(packed.is_ok());
    }

    #[test]
    fn roundtrip() {
        let arr = [1, 2, 3, 4, 5];
        let packed = arr.iter().try_pack().unwrap();
        let unpacked = packed.unpack();
        assert_eq!(&unpacked, &arr);
    }

    #[test]
    fn single_element() {
        let arr = [345u16];
        let packed = arr.iter().try_pack().unwrap();
        let unpacked = packed.unpack();
        assert_eq!(&unpacked, &arr);
    }

    #[test]
    fn max_slot_value() {
        let arr = [u12::MAX; 10];
        let packed = arr.iter().try_pack();
        assert!(packed.is_ok());
    }

    #[test]
    fn overflow_error() {
        let arr = [4096, 2, 3];
        let packed = arr.iter().try_pack();
        assert!(matches!(packed, Err(PackError::ItemOverflow(4096))));
    }

    #[test]
    fn over_max_items() {
        let arr = [1u16; 11];
        let packed = arr.iter().try_pack();
        assert!(matches!(packed, Err(PackError::Overflow(10))));
    }
}

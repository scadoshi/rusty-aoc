use crate::year_2025::day_10::u12::{
    BIT_LEN,
    pack::{Pack, len::GetPackLen},
};

pub trait Unpack {
    fn unpack(self) -> Vec<u16>;
}

impl Unpack for Pack {
    fn unpack(self) -> Vec<u16> {
        let len: usize = self.len();
        (0..len)
            .rev()
            .map(|i| u16::try_from((self >> i * BIT_LEN) & 0xFFF).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::year_2025::day_10::u12::pack::TryPack;
    #[test]
    fn unpack() {
        let input = [1, 2, 3, 4];
        let packed = input.iter().try_pack();
        assert!(packed.is_ok());
        let packed = packed.unwrap();
        let unpacked = packed.unpack();
        assert_eq!(unpacked, vec![1, 2, 3, 4])
    }
}

use crate::year_2025::day_10::u12::pack::{Pack, len::GetPackLen};

pub trait AnySlotsGreaterThan {
    fn any_slots_greater_than(&self, other: Self) -> bool;
}

impl AnySlotsGreaterThan for Pack {
    fn any_slots_greater_than(&self, other: Pack) -> bool {
        let len = other.len();
        let mut any_greater_than = false;
        for i in 0..len {
            let shift = (len - 1 - i) * 12;
            let mask = 0xFFFu128 << shift;
            let self_slot = (self & mask) >> shift;
            let other_slot = (other & mask) >> shift;
            if self_slot > other_slot {
                any_greater_than = true;
                break;
            }
        }
        any_greater_than
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::year_2025::day_10::u12::pack::TryPack;
    #[test]
    fn no_slots_greater_than() {
        let a = [1, 2, 3, 4].iter().try_pack().unwrap();
        let b = [1, 2, 3, 4].iter().try_pack().unwrap();
        assert!(!a.any_slots_greater_than(b));
    }
    #[test]
    fn all_slots_greater_than() {
        let a = [1, 2, 3, 4].iter().try_pack().unwrap();
        let b = [0, 0, 0, 0].iter().try_pack().unwrap();
        assert!(a.any_slots_greater_than(b));
    }
    #[test]
    fn one_slot_greater_than() {
        let a = [1, 2, 3, 4].iter().try_pack().unwrap();
        let b = [0, 2, 3, 4].iter().try_pack().unwrap();
        assert!(a.any_slots_greater_than(b));
    }
}

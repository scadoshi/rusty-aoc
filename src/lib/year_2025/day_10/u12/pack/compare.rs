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

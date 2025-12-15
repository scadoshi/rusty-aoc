use crate::year_2025::day_10::u12::pack::{ITEM_COUNT_MAX, len::GetPackLen};
type Button = u16;
pub trait Click {
    fn click(&self, button: Button) -> u128;
}

type State = u128;
impl Click for State {
    fn click(&self, button: u16) -> u128 {
        let len = self.len();
        let result = (0..len).fold(0u128, |updated, i| {
            let shift = (len - 1 - i) * 12;
            let mask = 0xFFFu128 << shift;
            let mut value = ((*self & mask) >> shift) as u16;
            if button & (1 << (len - 1 - i)) != 0 {
                value += 1;
            }
            updated | ((value as u128) << shift)
        });
        result | ((len as u128) << (ITEM_COUNT_MAX * 12))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_click_updates_state_correctly() {
        // pack state [1, 2] with len 2
        let value1: u16 = 1;
        let value2: u16 = 2;
        let len: usize = 2;
        let state =
            ((value1 as u128) << 12) | ((value2 as u128) << 0) | ((len as u128) << (10 * 12));

        // click high bit only
        let button: u16 = 0b10;
        let new_state = state.click(button);

        let new_value1 = ((new_state >> 12) & 0xFFF) as u16;
        let new_value2 = (new_state & 0xFFF) as u16;

        assert_eq!(new_value1, value1 + 1);
        assert_eq!(new_value2, value2);
        assert_eq!(new_state.len(), len); // len preserved

        // click low bit only
        let button_low: u16 = 0b01;
        let state2 = state.click(button_low);
        let new_value1b = ((state2 >> 12) & 0xFFF) as u16;
        let new_value2b = (state2 & 0xFFF) as u16;
        assert_eq!(new_value1b, value1);
        assert_eq!(new_value2b, value2 + 1);

        // click both
        let state3 = state.click(0b11);
        let new1 = ((state3 >> 12) & 0xFFF) as u16;
        let new2 = (state3 & 0xFFF) as u16;
        assert_eq!(new1, value1 + 1);
        assert_eq!(new2, value2 + 1);
    }
}

use crate::year_2025::day_10::u12::pack::{ITEM_COUNT_MAX, len::GetPackLen};

type State = u128;
type Button = u16;

pub trait Click {
    fn click(&self, button: Button) -> u128;
}

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
    use crate::year_2025::day_10::u12::pack::{TryPack, unpack::Unpack};
    #[test]
    fn simple_click() {
        let mut state = [0; 5].iter().try_pack().unwrap();
        let button: Button = 1;
        state = state.click(button);
        assert_eq!(state.unpack(), [0, 0, 0, 0, 1]);
    }

    #[test]
    fn multiple_clicks() {
        let mut state = [0; 5].iter().try_pack().unwrap();
        let button: Button = 1;
        for _ in 0..5 {
            state = state.click(button);
        }
        assert_eq!(state.unpack(), [0, 0, 0, 0, 5]);
    }
}

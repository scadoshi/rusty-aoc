use crate::year_2024::day_03::{DONT_PATTERN, DO_PATTERN};

pub trait DoOrDont {
    fn do_or_dont(&self, currently: bool) -> bool;
}

impl<T: AsRef<str>> DoOrDont for T {
    fn do_or_dont(&self, current: bool) -> bool {
        if DO_PATTERN.is_match(self.as_ref()) {
            true
        } else if DONT_PATTERN.is_match(self.as_ref()) {
            false
        } else {
            current
        }
    }
}

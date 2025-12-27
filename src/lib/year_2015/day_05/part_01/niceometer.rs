pub const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
pub const NAUGHTY: [(Option<u8>, Option<u8>); 4] = [
    (Some(b'a'), Some(b'b')),
    (Some(b'c'), Some(b'd')),
    (Some(b'p'), Some(b'q')),
    (Some(b'x'), Some(b'y')),
];

#[derive(Default, Debug)]
pub struct Niceometer {
    pub window: (Option<u8>, Option<u8>),
    pub vowel_count: u16,
    pub has_double: bool,
    pub has_naughty: bool,
}

impl Niceometer {
    pub fn new() -> Self {
        Niceometer::default()
    }

    pub fn is_nice(&self) -> bool {
        self.has_double && self.vowel_count >= 3 && !self.has_naughty
    }

    pub fn update_window(self, c: u8) -> Self {
        Self {
            window: (self.window.1, Some(c)),
            ..self
        }
    }

    pub fn update_has_double(self) -> Self {
        Self {
            has_double: self.has_double
                || (self.window.0 == self.window.1 && self.window.0.is_some()),
            ..self
        }
    }

    pub fn update_vowel_count(self, char: u8) -> Self {
        Self {
            vowel_count: self.vowel_count + u16::from(VOWELS.contains(&char)),
            ..self
        }
    }

    pub fn update_has_naughty(self) -> Self {
        Self {
            has_naughty: self.has_naughty || NAUGHTY.contains(&self.window),
            ..self
        }
    }
}

#[derive(Debug, Default)]
pub struct Niceometer {
    pub window: (Option<u8>, Option<u8>, Option<u8>),
    pub pairs: Vec<(u8, u8)>,
    pub pair_occurs_twice: bool,
    pub has_aba: bool,
}

impl Niceometer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_nice(&self) -> bool {
        self.pair_occurs_twice && self.has_aba
    }

    pub fn update_window(self, c: u8) -> Self {
        Self {
            window: (self.window.1, self.window.2, Some(c)),
            ..self
        }
    }

    pub fn update_pairs(self) -> Self {
        let (Some(first), Some(second)) = (self.window.1, self.window.2) else {
            return self;
        };
        let pair = (first, second);
        let mut new = self.pairs;
        let seen = new.contains(&pair);
        if !seen {
            new.push(pair)
        }
        let pair_occurs_twice = self.pair_occurs_twice || (seen && self.window.0 != self.window.1);
        Self {
            pairs: new,
            pair_occurs_twice,
            ..self
        }
    }

    pub fn update_has_aba(self) -> Self {
        Self {
            has_aba: self.has_aba || (self.window.0.is_some() && self.window.0 == self.window.2),
            ..self
        }
    }
}

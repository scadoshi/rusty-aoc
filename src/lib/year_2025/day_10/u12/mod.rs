pub mod error;
pub mod pack;

use crate::year_2025::day_10::u12::error::ParseU12Error;
use std::fmt::{Debug, Display};

pub const MAX: u16 = 4095;
pub const BIT_LEN: usize = 12;

#[derive(Clone, Copy)]
pub struct U12(u16);

impl U12 {
    pub fn value(self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for U12 {
    type Error = ParseU12Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > MAX {
            return Err(ParseU12Error::Overflow);
        }
        Ok(Self(value))
    }
}

impl From<u8> for U12 {
    fn from(value: u8) -> Self {
        Self(u16::from(value))
    }
}

impl Debug for U12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl Display for U12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <U12 as Debug>::fmt(self, f)
    }
}

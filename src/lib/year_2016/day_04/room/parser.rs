use std::collections::HashMap;

pub const CHECKSUM_LEN: u8 = 5;
pub const BYTE_LEN: u8 = 8;

#[derive(Debug, Clone, Copy, Default)]
pub enum IsParsing {
    #[default]
    Value,
    Id,
    Checksum,
}

#[derive(Debug, Clone, Default)]
pub struct Parser {
    pub is_parsing: IsParsing,
    pub encoded_name: String,
    pub counts: HashMap<u8, u8>,
    pub id: u32,
    pub checksum: u64,
    pub checksum_index: u8,
}

impl Parser {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }
    pub fn set_parsing_id(&mut self) -> &mut Self {
        self.is_parsing = IsParsing::Id;
        self
    }
    pub fn set_parsing_checksum(&mut self) -> &mut Self {
        self.is_parsing = IsParsing::Checksum;
        self
    }
    pub fn set_id_digit(&mut self, value: u8) -> &mut Self {
        if value.is_ascii_digit() {
            self.id = self.id * 10 + u32::from(value - b'0');
        }
        self
    }
    pub fn set_checksum_byte(&mut self, value: u8) -> &mut Self {
        if self.checksum_index == CHECKSUM_LEN {
            return self;
        }
        self.checksum |= u64::from(value) << (self.checksum_index * BYTE_LEN);
        self.checksum_index += 1;
        self
    }
    pub fn checksum_is_full(&self) -> bool {
        self.checksum_index == CHECKSUM_LEN
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_id() {
        let mut parser = Parser::new();
        parser
            .set_id_digit(b'1')
            .set_id_digit(b'2')
            .set_id_digit(b'3');
        assert_eq!(parser.id, 123);
    }
    #[test]
    fn set_one_byte_in_checksum() {
        let mut parser = Parser::new();
        parser.set_checksum_byte(b'a');
        assert_eq!(parser.checksum, 97);
    }
    #[test]
    fn set_all_bytes_in_checksum() {
        let mut parser = Parser::new();
        for _ in 0..5 {
            parser.set_checksum_byte(u8::MAX);
        }
        assert_eq!(parser.checksum, 1099511627775);
    }
    #[test]
    fn set_all_bytes_in_checksum_then_one() {
        let mut parser = Parser::new();
        for _ in 0..6 {
            parser.set_checksum_byte(u8::MAX);
        }
        assert_eq!(parser.checksum, 1099511627775);
    }
}

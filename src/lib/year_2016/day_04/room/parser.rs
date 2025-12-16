use std::collections::HashMap;

// const ID_LEN: u32 = 3;
pub const CHECKSUM_LEN: u8 = 5;
pub const BYTE_LEN: u8 = 8;

#[derive(Debug, Clone, Copy, Default)]
enum ParsingWhat {
    #[default]
    Value,
    Id,
    Checksum,
}

#[derive(Debug, Clone, Default)]
pub struct Parser {
    parsing: ParsingWhat,
    pub encoded_name: String,
    pub value_counts: HashMap<u8, u8>,
    pub id: u32,
    pub id_pow: u32,
    pub id_done: bool,
    pub checksum: u64,
    pub checksum_index: u8,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            id_pow: 2,
            ..Self::default()
        }
    }
    pub fn is_parsing_value(&self) -> bool {
        matches!(self.parsing, ParsingWhat::Value)
    }
    pub fn is_parsing_id(&self) -> bool {
        matches!(self.parsing, ParsingWhat::Id)
    }
    pub fn is_parsing_checksum(&self) -> bool {
        matches!(self.parsing, ParsingWhat::Checksum)
    }
    pub fn set_parsing_id(self) -> Self {
        Self {
            parsing: ParsingWhat::Id,
            ..self
        }
    }
    pub fn set_parsing_checksum(self) -> Self {
        Self {
            parsing: ParsingWhat::Checksum,
            ..self
        }
    }
    pub fn set_id_digit_from_num_char(self, value: u8) -> Self {
        if self.id_done {
            return self;
        }
        let mut id_done = false;
        Self {
            id: self.id + u32::from(value - b'0') * 10u32.pow(self.id_pow),
            id_pow: self.id_pow.checked_sub(1).unwrap_or_else(|| {
                id_done = true;
                0
            }),
            id_done,
            ..self
        }
    }
    pub fn set_checksum_byte(self, value: u8) -> Self {
        if self.checksum_index == CHECKSUM_LEN {
            return self;
        }
        Self {
            checksum: self.checksum | u64::from(value) << (self.checksum_index * BYTE_LEN),
            checksum_index: self.checksum_index + 1,
            ..self
        }
    }
    pub fn checksum_is_full(&self) -> bool {
        self.checksum_index == CHECKSUM_LEN
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_parsing_checks() {
        let parser = Parser::new();
        assert!(parser.is_parsing_value());
        let parser = parser.set_parsing_id();
        assert!(parser.is_parsing_id());
        let parser = parser.set_parsing_checksum();
        assert!(parser.is_parsing_checksum());
    }
    #[test]
    fn set_id() {
        let parser = Parser::new().set_id_digit_from_num_char(b'1');
        let parser = parser.set_id_digit_from_num_char(b'2');
        let parser = parser.set_id_digit_from_num_char(b'3');
        assert_eq!(parser.id, 123);
    }
    #[test]
    fn set_one_byte_in_checksum() {
        let parser = Parser::new().set_checksum_byte(b'a');
        assert_eq!(parser.checksum, 97);
    }
    #[test]
    fn set_all_bytes_in_checksum() {
        let mut parser = Parser::new();
        for _ in 0..5 {
            parser = parser.set_checksum_byte(u8::MAX);
        }
        assert_eq!(parser.checksum, 1099511627775);
    }
    #[test]
    fn set_all_bytes_in_checksum_then_one() {
        let mut parser = Parser::new();
        for _ in 0..6 {
            parser = parser.set_checksum_byte(u8::MAX);
        }
        assert_eq!(parser.checksum, 1099511627775);
    }
}

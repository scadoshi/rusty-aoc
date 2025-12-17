pub mod parser;

use crate::year_2016::day_04::room::parser::{BYTE_LEN, IsParsing, Parser};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Room {
    pub name: String,
    pub value_counts: HashMap<u8, u8>,
    pub id: u32,
    pub checksum: u64,
}

impl From<Parser> for Room {
    fn from(value: Parser) -> Self {
        let mut name: String = String::new();
        let incr = (value.id % 26) as u8;
        let len = value.encoded_name.len();
        for b in value.encoded_name.bytes().take(len - 1) {
            if b == b'-' {
                name.push('-');
            } else {
                name.push((((b - b'a' + incr) % 26) + b'a') as char)
            }
        }
        Self {
            name,
            value_counts: value.value_counts,
            id: value.id,
            checksum: value.checksum,
        }
    }
}

impl From<&str> for Room {
    fn from(value: &str) -> Self {
        value
            .bytes()
            .fold(Parser::new(), |mut parser, b| match parser.is_parsing {
                IsParsing::Value => {
                    if b == b'-' {
                        parser.encoded_name.push(b as char);
                    } else if b.is_ascii_digit() {
                        parser.set_parsing_id().set_id_digit(b);
                    } else {
                        parser.encoded_name.push(b as char);
                        *parser.value_counts.entry(b).or_default() += 1;
                    }
                    parser
                }
                IsParsing::Id => {
                    if b == b'[' {
                        parser.set_parsing_checksum();
                    } else {
                        parser.set_id_digit(b);
                    }
                    parser
                }
                IsParsing::Checksum => {
                    if !parser.checksum_is_full() {
                        parser.set_checksum_byte(b);
                    }
                    parser
                }
            })
            .into()
    }
}

impl Room {
    pub fn is_valid(&self) -> bool {
        let mut v: Vec<(&u8, &u8)> = self.value_counts.iter().collect();
        v.sort_by(|(ch1, co1), (ch2, co2)| co2.cmp(co1).then(ch1.cmp(ch2)));
        let generated_checksum: u64 = v
            .into_iter()
            .take(5)
            .enumerate()
            .fold(0u64, |acc, (i, (b, _))| {
                acc | u64::from(*b) << (i * BYTE_LEN as usize)
            });
        generated_checksum == self.checksum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::year_2016::day_04::room::parser::CHECKSUM_LEN;
    #[test]
    fn full_parse() {
        let result = Room::from("aaaaa-bbb-z-y-x-123[abxyz]");
        let expected = Room {
            name: "hhhhh-iii-g-f-e".to_string(),
            value_counts: HashMap::from([(b'a', 5), (b'b', 3), (b'x', 1), (b'y', 1), (b'z', 1)]),
            id: 123,
            checksum: {
                let value = [b'a', b'b', b'x', b'y', b'z'];
                (0..CHECKSUM_LEN).fold(0u64, |acc, i| {
                    acc | (u64::from(value[i as usize]) << i * BYTE_LEN)
                })
            },
        };
        assert_eq!(result, expected);
    }
    #[test]
    fn valid_room() {
        let r1 = Room::from("aaaaa-bbb-z-y-x-123[abxyz]");
        let r2 = Room::from("a-b-c-d-e-f-g-h-987[abcde]");
        let r3 = Room::from("not-a-real-room-404[oarel]");
        assert!(r1.is_valid() && r2.is_valid() && r3.is_valid());
    }
    #[test]
    fn invalid_room() {
        let r = Room::from("totally-real-room-200[decoy]");
        assert!(!r.is_valid());
    }
}

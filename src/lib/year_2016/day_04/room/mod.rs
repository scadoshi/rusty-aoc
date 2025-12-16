use crate::year_2016::day_04::room::parser::{BYTE_LEN, NUM_CHARS, Parser};
use std::collections::HashMap;

pub mod parser;

#[derive(Debug, Clone, PartialEq)]
pub struct Room {
    pub value_counts: HashMap<u8, u8>,
    pub id: u32,
    pub checksum: u64,
}

impl From<Parser> for Room {
    fn from(value: Parser) -> Self {
        Self {
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
            .fold(Parser::new(), |mut state, b| {
                if state.is_parsing_value() {
                    if b == b'-' {
                        return state;
                    }

                    if NUM_CHARS.contains(&b) {
                        return state.set_parsing_id().set_id_digit_from_num_char(b);
                    }

                    *state.value_counts.entry(b).or_default() += 1;
                }

                if state.is_parsing_id() {
                    if b == b'[' {
                        return state.set_parsing_checksum();
                    }
                    return state.set_id_digit_from_num_char(b);
                }

                if state.is_parsing_checksum() {
                    if state.checksum_is_full() {
                        return state;
                    }
                    return state.set_checksum_byte(b);
                }
                state.set_parsing_none()
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
    // #[test]
    // fn valid_room() {
    //     let r1 = Room::from("aaaaa-bbb-z-y-x-123[abxyz]");
    //     let r2 = Room::from("a-b-c-d-e-f-g-h-987[abcde]");
    //     assert!(r1.is_valid() && r2.is_valid());
    // }
    #[test]
    fn invalid_room() {
        let r1 = Room::from("not-a-real-room-404[oarel]");
        let r2 = Room::from("totally-real-room-200[decoy]");
        assert!(!r1.is_valid() && !r2.is_valid());
    }
}

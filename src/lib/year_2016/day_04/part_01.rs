use std::collections::HashMap;

const NUM_CHARS: [u8; 10] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
// const ID_LEN: u32 = 3;
const CHECKSUM_LEN: u8 = 5;

pub fn part_01(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            let (map, id, actual_checksum, ..) = line.bytes().fold(
                (
                    HashMap::<u8, u8>::new(),
                    0u32,
                    0u64,
                    true,
                    false,
                    false,
                    0u32,
                    0u8,
                ),
                |(
                    mut map,
                    id,
                    checksum,
                    parsing_val,
                    parsing_id,
                    parsing_checksum,
                    pow,
                    checksum_index,
                ),
                 b| {
                    if parsing_val {
                        if b == b'-' {
                            return (map, id, checksum, true, false, false, pow, checksum_index);
                        } else if NUM_CHARS.contains(&b) {
                            return (
                                map,
                                id + u32::from(b - b'0') * 10u32.pow(pow),
                                checksum,
                                false,
                                true,
                                false,
                                pow - 1,
                                checksum_index,
                            );
                        }
                        *map.entry(b).or_default() += 1;
                    } else if parsing_id {
                        if b == b'[' {
                            return (map, id, checksum, false, false, true, pow, checksum_index);
                        }
                        return (
                            map,
                            id + u32::from(b - b'0') * 10u32.pow(pow),
                            checksum,
                            false,
                            true,
                            false,
                            pow - 1,
                            checksum_index,
                        );
                    } else if parsing_checksum {
                        if checksum_index == CHECKSUM_LEN {
                            return (map, id, checksum, false, false, false, pow, checksum_index);
                        }
                        return (
                            map,
                            id,
                            checksum | u64::from(b) << checksum_index,
                            false,
                            false,
                            true,
                            pow,
                            checksum_index + 1,
                        );
                    }
                    (map, id, checksum, false, false, false, 0, 0)
                },
            );

            let mut v: Vec<(u8, u8)> = map.into_iter().collect();
            v.sort_by(|(ch1, co1), (ch2, co2)| co2.cmp(co1).then(ch1.cmp(ch2)));
            let generated_checksum: u64 = v
                .into_iter()
                .take(5)
                .enumerate()
                .fold(0u64, |acc, (i, (b, _))| acc | u64::from(b) << i);

            if generated_checksum == actual_checksum {
                id
            } else {
                0
            }
        })
        .sum()
}

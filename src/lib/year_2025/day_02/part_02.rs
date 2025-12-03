use std::ops::RangeInclusive;

pub fn part_02(input: &[RangeInclusive<usize>]) -> usize {
    input.iter().fold(0, |mut total, range| {
        range.clone().for_each(|id| {
            let id_str = id.to_string();
            let bytes = id_str.as_bytes();
            for i in 1..=id.to_string().len() / 2 {
                let first = &bytes[..i];
                if !bytes.chunks(i).any(|chunk| first != chunk) {
                    total += id;
                    break;
                }
            }
        });
        total
    })
}

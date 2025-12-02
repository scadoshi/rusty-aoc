use std::ops::RangeInclusive;

pub fn part_02(input: &[RangeInclusive<usize>]) -> usize {
    let mut total = 0;
    for range in input {
        'ids: for id in range.clone() {
            let id_str = id.to_string();
            let bytes = id_str.as_bytes();
            for i in 1..=id.to_string().len() / 2 {
                let first = &bytes[..i];
                if bytes.chunks(i).all(|chunk| chunk == first) {
                    total += id;
                    continue 'ids;
                }
            }
        }
    }
    total
}

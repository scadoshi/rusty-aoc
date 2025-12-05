fn hashed_leading_zero_count(value: &str, append_with: usize) -> usize {
    format!(
        "{:x}",
        md5::compute(format!("{}{}", value.trim(), append_with))
    )
    .chars()
    .take_while(|c| *c == '0')
    .count()
}

pub trait AdventHasher {
    fn advent_hasher<const T: usize>(self) -> usize;
}

impl AdventHasher for &str {
    fn advent_hasher<const T: usize>(self) -> usize {
        let mut append_with = 0;
        while hashed_leading_zero_count(self, append_with) < T {
            append_with += 1;
        }
        append_with
    }
}

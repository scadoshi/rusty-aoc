use std::collections::HashSet;

pub fn part_01(input: &[&'static str]) -> usize {
    input.into_iter().fold(0usize, |acc, line| {
        let mut seen = HashSet::<&[u8]>::new();
        for word in line.split_whitespace().map(|w| w.as_bytes()) {
            if seen.contains(word) {
                return acc;
            } else {
                seen.insert(&word);
            }
        }
        acc + 1
    })
}

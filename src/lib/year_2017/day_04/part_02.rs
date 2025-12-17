use std::collections::HashSet;

pub fn part_02(input: &[&'static str]) -> u16 {
    input.into_iter().fold(0u16, |acc, line| {
        let mut seen = HashSet::<Vec<char>>::new();
        for word in line.split_whitespace() {
            let mut v = word.chars().collect::<Vec<char>>();
            v.sort();
            if seen.contains(&v) {
                return acc;
            } else {
                seen.insert(v);
            }
        }
        acc + 1
    })
}

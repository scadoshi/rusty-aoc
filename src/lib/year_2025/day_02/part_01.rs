use std::ops::RangeInclusive;

pub fn part_01(input: &[RangeInclusive<usize>]) -> usize {
    input
        .into_iter()
        .map(|range| {
            range
                .clone()
                .filter(|num| {
                    let num_str = num.to_string();
                    let (left, right) = num_str.split_at(num_str.len() / 2);
                    left == right
                })
                .sum::<usize>()
        })
        .sum()
}

use std::ops::RangeInclusive;

pub fn get_input() -> Vec<RangeInclusive<usize>> {
    include_str!("input.txt")
        .split(',')
        .map(|x| {
            let (num1, num2) = x.split_once('-').unwrap();
            num1.trim().parse().unwrap()..=num2.trim().parse().unwrap()
        })
        .collect()
}

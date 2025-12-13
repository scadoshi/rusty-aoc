use std::collections::{HashSet, VecDeque};

pub fn part_02(input: &[(u16, Vec<u16>, Vec<u16>)]) -> usize {
    input
        .iter()
        .filter_map(|line| {
            let (_lights, buttons, target) = line;
            let mut seen = HashSet::<Vec<u16>>::new();
            let queue = VecDeque::<Vec<u16>>::from([vec![0; target.len()]]);

            Some(0)
        })
        .sum()
}

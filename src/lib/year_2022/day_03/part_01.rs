use crate::year_2022::day_03::priority::Priority;

pub fn part_01(input: &[String]) -> usize {
    input
        .iter()
        .map(|x| {
            let left = &x[..x.len() / 2];
            let right = &x[x.len() / 2..];
            let common = left.chars().find(|c| right.contains(*c)).unwrap();
            common.priority()
        })
        .sum()
}

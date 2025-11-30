use crate::year_2018::day_03::claim::Claim;

pub fn get_input() -> Vec<Claim> {
    include_str!("input.txt")
        .lines()
        .map(|line| Claim::from(line))
        .collect()
}

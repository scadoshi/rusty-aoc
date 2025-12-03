use crate::year_2024::day_05::sequence::{Rule, RulesOps, Sequence};

pub fn part_01(input: &(Vec<Rule>, Vec<Sequence>)) -> usize {
    let (rules, sequences) = input;
    sequences
        .iter()
        .filter(|s| s.follows_rules(&rules))
        .map(|s| usize::from(*s.get(s.len() / 2).unwrap()))
        .sum()
}

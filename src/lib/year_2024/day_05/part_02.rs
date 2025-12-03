use crate::year_2024::day_05::sequence::{Rule, RulesOps, Sequence};

pub fn part_02(input: &(Vec<Rule>, Vec<Sequence>)) -> usize {
    let (rules, sequences) = input;
    sequences
        .iter()
        .filter(|s| !s.follows_rules(&rules))
        .map(|s| {
            let mut ss = s.clone();
            ss.sort_by_rules(rules);
            usize::from(*ss.get(ss.len() / 2).unwrap())
        })
        .sum()
}

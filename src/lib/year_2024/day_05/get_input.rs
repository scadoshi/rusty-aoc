use crate::year_2024::day_05::sequence::{Rule, Sequence};

pub fn get_input() -> (Vec<Rule>, Vec<Sequence>) {
    let (rules, sequences) = include_str!("input.txt").split_once("\n\n").unwrap();
    (
        rules.lines().map(Rule::from).collect(),
        sequences
            .lines()
            .map(|l| l.split(',').map(|x| x.parse::<u8>().unwrap()).collect())
            .collect(),
    )
}

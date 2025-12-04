use crate::year_2020::day_02::password_policy::PasswordAndPolicy;

pub fn part_01(input: &[PasswordAndPolicy]) -> usize {
    input
        .iter()
        .filter(|pap| pap.range_requirements_met())
        .count()
}

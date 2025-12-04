use crate::year_2020::day_02::password_policy::PasswordAndPolicy;

pub fn part_02(input: &[PasswordAndPolicy]) -> usize {
    input
        .iter()
        .filter(|pap| pap.position_requirements_met())
        .count()
}

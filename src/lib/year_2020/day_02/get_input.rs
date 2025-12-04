use crate::year_2020::day_02::password_policy::PasswordAndPolicy;

pub fn get_input() -> Vec<PasswordAndPolicy> {
    include_str!("input.txt")
        .lines()
        .map(PasswordAndPolicy::from)
        .collect()
}

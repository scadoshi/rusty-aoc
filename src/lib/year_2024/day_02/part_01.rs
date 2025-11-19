use crate::year_2024::day_02::is_safe::IsSafe;

pub fn part_01(input: &[Vec<i32>]) -> usize {
    input.iter().filter(|r| r.is_safe()).count()
}

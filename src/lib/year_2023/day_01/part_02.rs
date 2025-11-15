use crate::year_2023::day_01::find_number::FindNumber;

#[allow(dead_code)]
pub fn part_02(input: &[String]) -> Option<i32> {
    input
        .iter()
        .map(|s| s.as_str().both_numbers())
        .collect::<Option<Vec<i32>>>()
        .map(|v| v.iter().sum())
}

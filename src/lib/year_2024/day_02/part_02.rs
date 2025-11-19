use crate::year_2024::day_02::is_safe::IsSafe;

pub fn part_02(input: &[Vec<i32>]) -> usize {
    input
        .iter()
        .filter(|r| {
            if r.is_safe() {
                return true;
            }
            for i in 0..r.len() {
                let new: Vec<i32> = r
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, num)| *num)
                    .collect();
                if new.is_safe() {
                    return true;
                }
            }
            false
        })
        .count()
}

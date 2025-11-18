#[allow(dead_code)]
pub fn part_02(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .filter_map(|line| {
            for (i, num1) in line.iter().enumerate() {
                if let Some((_, num2)) = line
                    .iter()
                    .enumerate()
                    .find(|(j, num2)| i != *j && *num1 % **num2 == 0)
                {
                    return Some(*num1 / *num2);
                }
            }
            None
        })
        .sum()
}

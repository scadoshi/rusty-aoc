#[allow(dead_code)]
pub fn part_02(input: &[i32]) -> i32 {
    input
        .windows(3)
        .map(|window| window.to_vec())
        .collect::<Vec<Vec<i32>>>()
        .windows(2)
        .filter(|window| window[1].iter().sum::<i32>() > window[0].iter().sum::<i32>())
        .count() as i32
}

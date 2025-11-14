#[allow(dead_code)]
pub fn part_01(input: &[i32]) -> i32 {
    input
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count() as i32
}

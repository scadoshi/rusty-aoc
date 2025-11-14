#[allow(dead_code)]
pub fn part_02(input: &[u8]) -> i32 {
    input
        .iter()
        .enumerate()
        .filter(|(i, num)| {
            let next = input[(i + input.len() / 2) % input.len()];
            **num == next
        })
        .map(|(_, num)| *num as i32)
        .sum()
}

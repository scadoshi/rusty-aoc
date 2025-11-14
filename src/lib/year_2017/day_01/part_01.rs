#[allow(dead_code)]
pub fn part_01(input: &[u8]) -> i32 {
    input
        .iter()
        .enumerate()
        .filter(|(i, num)| {
            let next = if *i == input.len() - 1 {
                input[0]
            } else {
                input[i + 1]
            };
            **num == next
        })
        .map(|(_, num)| *num as i32)
        .sum()
}

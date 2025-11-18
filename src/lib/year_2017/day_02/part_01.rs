#[allow(dead_code)]
pub fn part_01(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|nums| {
            let min = nums.iter().min().expect("min not found");
            let max = nums.iter().max().expect("max not found");
            max - min
        })
        .sum()
}

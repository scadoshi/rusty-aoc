#[allow(dead_code)]
pub fn part_01(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|e| e.iter().sum())
        .max()
        .expect("max not found")
}

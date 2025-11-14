#[allow(dead_code)]
pub fn part_02(input: &[Vec<i32>]) -> i32 {
    let mut elves: Vec<i32> = input.iter().map(|e| e.iter().sum()).collect();
    elves.sort();
    elves.iter().rev().take(3).sum()
}

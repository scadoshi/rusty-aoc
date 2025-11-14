use std::collections::HashSet;

#[allow(dead_code)]
pub fn part_02(input: &[i32]) -> i32 {
    let mut frequency = 0;
    let mut visited = HashSet::from([0]);
    let mut p = 0;
    loop {
        frequency += input[p];
        if visited.contains(&frequency) {
            return frequency;
        } else {
            visited.insert(frequency);
        }
        p = (p + 1) % input.len();
    }
}

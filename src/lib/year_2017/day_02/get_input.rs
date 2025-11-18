#[allow(dead_code)]
pub fn get_input() -> Vec<Vec<i32>> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().expect("failed to parse i32"))
                .collect()
        })
        .collect()
}

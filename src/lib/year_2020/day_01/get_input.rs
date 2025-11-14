#[allow(dead_code)]
pub fn get_input() -> Vec<i32> {
    include_str!("input.txt")
        .lines()
        .map(|x| x.parse::<i32>().expect("failed to parse i32"))
        .collect()
}

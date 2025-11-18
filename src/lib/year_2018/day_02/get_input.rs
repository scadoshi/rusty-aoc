#[allow(dead_code)]
pub fn get_input() -> Vec<String> {
    include_str!("input.txt")
        .lines()
        .map(|x| x.to_string())
        .collect()
}

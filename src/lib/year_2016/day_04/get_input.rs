pub fn get_input() -> Vec<String> {
    include_str!("input.txt")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

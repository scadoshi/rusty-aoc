pub fn get_input() -> Vec<Vec<char>> {
    include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

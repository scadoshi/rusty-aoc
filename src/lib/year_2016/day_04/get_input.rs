pub fn get_input() -> Vec<&'static str> {
    include_str!("input.txt")
        .lines()
        .map(|x| x)
        .collect()
}

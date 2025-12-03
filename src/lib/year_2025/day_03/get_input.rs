pub fn get_input() -> Vec<Vec<u8>> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

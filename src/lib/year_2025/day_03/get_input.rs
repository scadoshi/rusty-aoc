pub fn get_input() -> Vec<Vec<u8>> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
                .collect::<Vec<u8>>()
        })
        .collect()
}

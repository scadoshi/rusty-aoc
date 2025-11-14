#[allow(dead_code)]
pub fn get_input() -> Vec<u8> {
    include_str!("input.txt")
        .chars()
        .map(|x| x.to_digit(10).expect("failed to parse digit") as u8)
        .collect()
}

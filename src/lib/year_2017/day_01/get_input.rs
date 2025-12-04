#[allow(dead_code)]
pub fn get_input() -> Vec<u8> {
    include_str!("input.txt")
        .chars()
        .map(|x| u8::try_from(x.to_digit(10).unwrap()).unwrap())
        .collect()
}

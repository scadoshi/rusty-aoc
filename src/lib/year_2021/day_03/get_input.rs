pub fn get_input() -> Vec<u16> {
    include_str!("input.txt")
        .lines()
        .map(|x| u16::from_str_radix(x, 2).unwrap())
        .collect()
}

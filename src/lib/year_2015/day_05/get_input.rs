pub fn get_input() -> Vec<&'static [u8]> {
    include_str!("input.txt")
        .lines()
        .map(|x| x.as_bytes())
        .collect()
}

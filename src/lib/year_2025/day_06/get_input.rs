pub fn get_input() -> &'static str {
    include_str!("input.txt")
}

pub trait GetInput {
    fn ops(self) -> impl Iterator<Item = u8>;
}

impl GetInput for &'static str {
    fn ops(self) -> impl Iterator<Item = u8> {
        self.lines()
            .last()
            .unwrap()
            .bytes()
            .into_iter()
            .filter(|b| *b != b' ')
    }
}

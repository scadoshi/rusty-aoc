pub fn get_input() -> usize {
    include_str!("input.txt").trim().parse::<usize>().unwrap()
}

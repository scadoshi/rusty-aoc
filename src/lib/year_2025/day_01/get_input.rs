#[allow(dead_code)]
pub fn get_input() -> Vec<isize> {
    include_str!("input.txt")
        .lines()
        .map(|x| x.replace("R", "").replace("L", "-").parse().unwrap())
        .collect()
}

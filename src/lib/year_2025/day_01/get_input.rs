#[allow(dead_code)]
pub fn get_input() -> Vec<i32> {
    include_str!("input.txt")
        .lines()
        .map(|x| x.replace("R", "").replace("L", "-").parse().unwrap())
        .collect()
}

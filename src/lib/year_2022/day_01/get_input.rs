#[allow(dead_code)]
pub fn get_input() -> Vec<Vec<i32>> {
    include_str!("input.txt")
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|x| x.parse::<i32>().expect("failed to parse i32"))
                .collect()
        })
        .collect()
}

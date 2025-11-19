pub fn get_input() -> Vec<Vec<i32>> {
    include_str!("input.txt")
        .lines()
        .map(|report_str| {
            report_str
                .split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

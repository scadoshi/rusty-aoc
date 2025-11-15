#[allow(dead_code)]
pub fn part_01(input: &[String]) -> i32 {
    input
        .iter()
        .map(|x| {
            let nums = x
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            let first = nums.iter().take(1).next().expect("failed to get next");
            let last = nums
                .iter()
                .rev()
                .take(1)
                .next()
                .expect("failed to get next");
            (format!("{}{}", first, last).as_str())
                .parse::<i32>()
                .expect("failed to parse i32")
        })
        .sum()
}

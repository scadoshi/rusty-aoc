use crate::year_2025::day_06::get_input::GetInput;

pub fn part_01(input: &'static str) -> usize {
    let mut ops = input.ops();
    let lines = input.lines();
    let len = lines.clone().count().checked_sub(1).unwrap();
    let lines: Vec<Vec<usize>> = lines
        .take(len)
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    (0..lines.get(0).unwrap().len())
        .map(|i| {
            let nums = lines.iter().map(|list| *list.get(i).unwrap());
            match ops.nth(0).unwrap() {
                b'*' => nums.product(),
                b'+' => nums.sum(),
                _ => {
                    panic!();
                    #[allow(unreachable_code)]
                    0
                }
            }
        })
        .sum()
}

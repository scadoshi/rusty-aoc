use crate::year_2025::day_06::get_input::GetInput;

pub fn part_02(input: &'static str) -> usize {
    let mut ops = input.ops();
    let lines = input.lines();
    let len = lines.clone().count().checked_sub(1).unwrap();
    let lines: Vec<Vec<char>> = lines.take(len).map(|line| line.chars().collect()).collect();
    (0..lines.iter().map(|row| row.len()).max().unwrap())
        .map(|i| lines.iter().filter_map(|r| r.get(i)).collect::<String>())
        .fold(
            Vec::<Vec<usize>>::from([Vec::new()]),
            |mut groups, column| {
                if let Ok(num) = column.trim().parse::<usize>() {
                    let last = groups.len().checked_sub(1).unwrap();
                    groups.get_mut(last).unwrap().push(num);
                } else {
                    groups.push(Vec::new());
                }
                groups
            },
        )
        .iter()
        .map(|nums| match ops.next().unwrap() {
            b'*' => nums.iter().product(),
            b'+' => nums.iter().sum(),
            _ => {
                panic!();
                #[allow(unreachable_code)]
                0
            }
        })
        .sum()
}

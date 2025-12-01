use crate::year_2022::day_03::priority::Priority;

pub fn part_02(input: &[String]) -> usize {
    input
        .chunks(3)
        .map(|chunk| {
            let common = chunk[0]
                .chars()
                .find(|c| chunk[1].contains(*c) && chunk[2].contains(*c))
                .unwrap();
            common.priority()
        })
        .sum()
}

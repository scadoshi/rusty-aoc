pub fn part_01(input: &(Vec<(usize, usize)>, Vec<usize>)) -> usize {
    let (ranges, values) = input;
    values
        .iter()
        .filter(|value| {
            ranges
                .iter()
                .any(|&(start, end)| **value >= start && **value <= end)
        })
        .count()
}

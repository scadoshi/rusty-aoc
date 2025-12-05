use range_set_blaze::RangeSetBlaze;

pub fn part_01(input: &(RangeSetBlaze<usize>, Vec<usize>)) -> usize {
    let (range, values) = input;
    values
        .iter()
        .filter(|value| range.contains(**value))
        .count()
}

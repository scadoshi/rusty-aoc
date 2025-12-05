use range_set_blaze::RangeSetBlaze;

pub fn part_02(input: &(RangeSetBlaze<usize>, Vec<usize>)) -> usize {
    let (range, _) = input;
    range.len().try_into().unwrap()
}

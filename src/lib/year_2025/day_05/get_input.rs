use range_set_blaze::RangeSetBlaze;

pub fn get_input() -> (RangeSetBlaze<usize>, Vec<usize>) {
    let (ranges, values) = include_str!("input.txt").split_once("\n\n").unwrap();
    (
        ranges
            .trim()
            .lines()
            .map(|rng_str| {
                let (start, end) = rng_str.split_once('-').unwrap();
                let (start, end) = (
                    start.parse::<usize>().unwrap(),
                    end.parse::<usize>().unwrap(),
                );
                start..=end
            })
            .collect(),
        values
            .trim()
            .lines()
            .map(|v| v.parse::<usize>().unwrap())
            .collect(),
    )
}

pub fn get_input() -> (Vec<(usize, usize)>, Vec<usize>) {
    let (ranges, values) = include_str!("input.txt").split_once("\n\n").unwrap();
    (
        ranges
            .trim()
            .lines()
            .map(|range_string| {
                let (str1, str2) = range_string.split_once('-').unwrap();
                let (num1, num2) = (
                    str1.parse::<usize>().unwrap(),
                    str2.parse::<usize>().unwrap(),
                );
                (num1.min(num2), num1.max(num2))
            })
            .collect(),
        values
            .trim()
            .lines()
            .map(|v| v.parse::<usize>().unwrap())
            .collect(),
    )
}

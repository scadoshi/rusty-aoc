#[allow(dead_code)]
pub fn part_01(input: &[isize]) -> isize {
    input
        .iter()
        .fold((50, 0), |(mut p, mut t), num| {
            p = (p + num) % 100;
            if p < 0 {
                p += 100;
            }
            if p == 0 {
                t += 1;
            }
            (p, t)
        })
        .1
}

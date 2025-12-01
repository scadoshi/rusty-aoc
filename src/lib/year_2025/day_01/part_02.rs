#[allow(dead_code)]
pub fn part_02(input: &[isize]) -> isize {
    input
        .into_iter()
        .fold((50, 0), |(mut p, mut t), num| {
            for _ in 0..num.abs() {
                p = (p + num.signum()) % 100;
                if p < 0 {
                    p += 100;
                }
                if p == 0 {
                    t += 1;
                }
            }
            (p, t)
        })
        .1
}

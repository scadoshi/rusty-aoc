use crate::year_2025::day_01::DIAL_SIZE;

#[allow(dead_code)]
pub fn part_02(input: &[isize]) -> isize {
    input
        .iter()
        .fold((50, 0), |(mut p, mut t), num| {
            for _ in 0..num.abs() {
                p = (p + num.signum() + DIAL_SIZE) % DIAL_SIZE;
                if p == 0 {
                    t += 1;
                }
            }
            (p, t)
        })
        .1
}

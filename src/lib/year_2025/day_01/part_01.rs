use crate::year_2025::day_01::DIAL_SIZE;

#[allow(dead_code)]
pub fn part_01(input: &[isize]) -> isize {
    input
        .iter()
        .fold((50, 0), |(mut p, mut t), num| {
            p = (p + num + DIAL_SIZE) % DIAL_SIZE;
            if p == 0 {
                t += 1;
            }
            (p, t)
        })
        .1
}

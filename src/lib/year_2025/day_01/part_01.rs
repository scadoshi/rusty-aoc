#[allow(dead_code)]
pub fn part_01(input: &[isize]) -> isize {
    let mut p = 50;
    let mut t = 0;
    for num in input {
        p = (p + num) % 100;
        if p < 0 {
            p += 100;
        }
        if p == 0 {
            t += 1;
        }
    }
    t
}

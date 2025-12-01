#[allow(dead_code)]
pub fn part_01(input: &[i32]) -> i32 {
    let mut p = 50;
    let mut total = 0;
    for num in input {
        p = (p + num) % 100;
        if p < 0 {
            p += 100;
        }
        if p == 0 {
            total += 1;
        }
    }
    total
}

#[allow(dead_code)]
pub fn get_input() -> String {
    include_str!("day_01_input.txt").to_string()
}

#[allow(dead_code)]
pub fn part_01(input: &str) -> i32 {
    input.chars().fold(0, |mut f, char| {
        if char == '(' {
            f += 1;
        } else if char == ')' {
            f -= 1
        };
        f
    })
}

#[allow(dead_code)]
pub fn part_02(input: &str) -> Option<usize> {
    let mut f = 0;
    for (i, char) in input.chars().enumerate() {
        if char == '(' {
            f += 1;
        } else if char == ')' {
            f -= 1;
        }
        if f == -1 {
            return Some(i + 1);
        }
    }
    None
}

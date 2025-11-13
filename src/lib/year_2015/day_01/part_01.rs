#[allow(dead_code)]
pub fn part_01(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (vec1, vec2) = input;
    vec1.iter()
        .zip(vec2)
        .map(|(num1, num2)| (num1 - num2).abs())
        .sum()
}

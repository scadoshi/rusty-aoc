#[allow(dead_code)]
pub fn get_input() -> (Vec<i32>, Vec<i32>) {
    let (mut vec1, mut vec2) = include_str!("input.txt").lines().fold(
        (Vec::new(), Vec::new()),
        |(mut vec1, mut vec2), line| {
            let (str1, str2) = line.split_once("   ").expect("failed to split once");
            let (num1, num2) = (
                str1.parse::<i32>().expect("failed to parse i32"),
                str2.parse::<i32>().expect("failed to parse i32"),
            );
            vec1.push(num1);
            vec2.push(num2);
            (vec1, vec2)
        },
    );
    vec1.sort();
    vec2.sort();
    (vec1, vec2)
}

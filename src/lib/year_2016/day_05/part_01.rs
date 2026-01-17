use crate::year_2016::day_05::append_and_hash::AppendAndHash;

pub fn part_01(input: &'static str) -> String {
    let mut append_with = 0;
    let mut password = String::new();

    while password.len() < 8 {
        let hashed = input.append_and_hash(append_with);
        if hashed.starts_with("00000") {
            password.push(hashed.chars().nth(5).unwrap());
        }
        append_with += 1;
    }
    password
}

use crate::year_2016::day_05::append_and_hash::AppendAndHash;

pub fn part_02(input: &'static str) -> String {
    let mut append_with = 0;
    let mut password: Vec<char> = vec!['_'; 8];
    while password.contains(&'_') {
        let hashed = input.append_and_hash(append_with);
        if hashed.starts_with("00000")
            && let Some(i) = hashed.chars().nth(5).unwrap().to_digit(10)
            && i < password.len() as u32
            && password[i as usize] == '_'
        {
            password[i as usize] = hashed.chars().nth(6).unwrap();
        }
        append_with += 1;
    }
    password.iter().collect::<String>()
}

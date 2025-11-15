use regex::Regex;

fn digits_regex() -> Vec<(i32, Regex)> {
    vec![
        (0, Regex::new(".*zero.*").expect("failed to create regex")),
        (1, Regex::new(".*one.*").expect("failed to create regex")),
        (2, Regex::new(".*two.*").expect("failed to create regex")),
        (3, Regex::new(".*three.*").expect("failed to create regex")),
        (4, Regex::new(".*four.*").expect("failed to create regex")),
        (5, Regex::new(".*five.*").expect("failed to create regex")),
        (6, Regex::new(".*six.*").expect("failed to create regex")),
        (7, Regex::new(".*seven.*").expect("failed to create regex")),
        (8, Regex::new(".*eight.*").expect("failed to create regex")),
        (9, Regex::new(".*nine.*").expect("failed to create regex")),
    ]
}

pub fn first_num(value: &str) -> Option<i32> {
    let digits_regex = digits_regex();
    let mut first: Option<i32> = None;

    // so ready to solve this bro
    // i know it
    // lfg
    // do it right here

    first
}

pub fn last_num(_value: &str) -> Option<i32> {
    let _digits_regex = digits_regex();
    None
}

#[allow(dead_code)]
pub fn part_02(_input: &[String]) -> i32 {
    let _debug_input: Vec<String> = "two1nine".lines().map(|x| x.to_string()).collect();

    0
}

use regex::Regex;

pub fn first_last(value: &str) -> Option<i32> {
    let patterns = vec![
        (Regex::new(".*zero.*").expect("failed to create regex"), 0),
        (Regex::new(".*one.*").expect("failed to create regex"), 1),
        (Regex::new(".*two.*").expect("failed to create regex"), 2),
        (Regex::new(".*three.*").expect("failed to create regex"), 3),
        (Regex::new(".*four.*").expect("failed to create regex"), 4),
        (Regex::new(".*five.*").expect("failed to create regex"), 5),
        (Regex::new(".*six.*").expect("failed to create regex"), 6),
        (Regex::new(".*seven.*").expect("failed to create regex"), 7),
        (Regex::new(".*eight.*").expect("failed to create regex"), 8),
        (Regex::new(".*nine.*").expect("failed to create regex"), 9),
    ];

    let mut first: Option<i32> = None;
    'main: for window in value.chars().collect::<Vec<char>>().windows(5) {
        for (pattern, num) in patterns.iter() {
            let str = window.iter().collect::<String>();
            if pattern.is_match(str.as_str()) {
                first = Some(*num);
                break 'main;
            }
            if let Some(num) = window.iter().filter_map(|c| c.to_digit(10)).take(1).next() {
                first = Some(num as i32);
                break 'main;
            }
        }
    }

    let mut last: Option<i32> = None;
    'main: for window in value.chars().collect::<Vec<char>>().windows(5).rev() {
        for (pattern, num) in patterns.iter() {
            if pattern.is_match(window.iter().collect::<String>().as_str()) {
                last = Some(*num);
                break 'main;
            }
            if let Some(num) = window
                .iter()
                .rev()
                .filter_map(|c| c.to_digit(10))
                .take(1)
                .next()
            {
                last = Some(num as i32);
                break 'main;
            }
        }
    }

    if let Some(first) = first {
        if let Some(last) = last {
            format!("{}{}", first, last).parse::<i32>().ok()
        } else {
            None
        }
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn part_02(_input: &[String]) -> i32 {
    let debug_input: Vec<String> = "two1nine".lines().map(|x| x.to_string()).collect();

    for line in debug_input.iter() {
        println!("{:?}", first_last(line));
    }

    0
}

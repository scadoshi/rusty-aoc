use crate::year_2016::day_02::{direction::Direction, keypad::KeypadPoint};

pub fn part_01(input: &[Vec<Direction>]) -> i32 {
    let (code, _) = input.iter().fold(
        (String::new(), KeypadPoint::default()),
        |(mut code, mut keypad_point), directions| {
            for direction in directions {
                keypad_point.traverse(direction);
            }
            code.push(keypad_point.to_char());
            (code, keypad_point)
        },
    );
    code.parse::<i32>().expect("failed to parse i32")
}

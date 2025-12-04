use crate::{common::direction::Direction, year_2016::day_02::keypad::Keypad};

pub fn part_02(input: &[Vec<Direction>]) -> String {
    let (code, _) = input.iter().fold(
        (String::new(), Keypad::weird()),
        |(mut code, mut keypad), directions| {
            for direction in directions {
                keypad.traverse(*direction);
            }
            if let Some(char) = keypad.get_char() {
                code.push(char);
            }
            (code, keypad)
        },
    );
    code
}

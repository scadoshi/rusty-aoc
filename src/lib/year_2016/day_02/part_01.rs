use crate::year_2016::day_02::{direction::Direction, keypad::Keypad};

pub fn part_01(input: &[Vec<Direction>]) -> String {
    let (code, _) = input.iter().fold(
        (String::new(), Keypad::standard()),
        |(mut code, mut keypad), directions| {
            for direction in directions {
                keypad.traverse(direction);
            }
            if let Some(char) = keypad.get_char() {
                code.push(char);
            }
            (code, keypad)
        },
    );
    code
}

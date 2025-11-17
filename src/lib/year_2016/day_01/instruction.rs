use crate::year_2016::day_01::directions::TurningDirection;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub direction: TurningDirection,
    pub distance: i32,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let direction = match value
            .to_lowercase()
            .trim()
            .chars()
            .take(1)
            .collect::<String>()
            .as_str()
        {
            "l" => TurningDirection::Left,
            "r" => TurningDirection::Right,
            x => panic!("invalid turning direction {:?}", x),
        };
        let distance = value
            .chars()
            .skip(1)
            .collect::<String>()
            .trim()
            .parse::<i32>()
            .expect("failed to parse i32");
        Self {
            direction,
            distance,
        }
    }
}

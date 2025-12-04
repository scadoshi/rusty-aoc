use crate::year_2016::day_01::turn::TurnDirection;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub direction: TurnDirection,
    pub distance: isize,
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
            "l" => TurnDirection::Left,
            "r" => TurnDirection::Right,
            x => panic!("invalid turning direction {:?}", x),
        };
        let distance = value
            .chars()
            .skip(1)
            .collect::<String>()
            .trim()
            .parse::<isize>()
            .unwrap();
        Self {
            direction,
            distance,
        }
    }
}

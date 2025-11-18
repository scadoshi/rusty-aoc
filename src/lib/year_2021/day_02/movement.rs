#[derive(Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            x => panic!("{} is not a valid direction", x),
        }
    }
}

#[derive(Debug)]
pub struct Movement {
    pub direction: Direction,
    pub distance: i32,
}

impl From<&str> for Movement {
    fn from(value: &str) -> Self {
        // e.g.
        // forward 7
        let (direction_str, distance_str) = value.split_once(" ").expect("failed to split once");
        let (direction, distance) = (
            Direction::from(direction_str),
            distance_str.parse::<i32>().expect("failed to parse i32"),
        );
        Movement {
            direction,
            distance,
        }
    }
}

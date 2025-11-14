#[derive(Debug, Clone)]
pub enum TurningDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum FacingDirection {
    Left,
    Right,
    #[default]
    Up,
    Down,
}

impl FacingDirection {
    pub fn turn(&mut self, direction: &TurningDirection) {
        match direction {
            TurningDirection::Left => match self {
                FacingDirection::Left => *self = FacingDirection::Down,
                FacingDirection::Right => *self = FacingDirection::Up,
                FacingDirection::Up => *self = FacingDirection::Left,
                FacingDirection::Down => *self = FacingDirection::Right,
            },
            TurningDirection::Right => match self {
                FacingDirection::Left => *self = FacingDirection::Up,
                FacingDirection::Right => *self = FacingDirection::Down,
                FacingDirection::Up => *self = FacingDirection::Right,
                FacingDirection::Down => *self = FacingDirection::Left,
            },
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn distance_from_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct DirectionalPoint {
    pub point: Point,
    pub facing_direction: FacingDirection,
}

impl DirectionalPoint {
    pub fn turn(&mut self, direction: &TurningDirection) {
        self.facing_direction.turn(direction)
    }

    pub fn travel(&mut self, distance: i32) {
        match self.facing_direction {
            FacingDirection::Left => self.point.x -= distance,
            FacingDirection::Right => self.point.x += distance,
            FacingDirection::Down => self.point.y -= distance,
            FacingDirection::Up => self.point.y += distance,
        }
    }

    pub fn step(&mut self) {
        self.travel(1);
    }
}

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

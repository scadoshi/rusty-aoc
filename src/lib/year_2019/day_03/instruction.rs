use crate::{
    common::{cartesian_point::Point, direction::Direction},
    year_2019::day_03::point::PointWithSteps,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Instruction {
    pub direction: Direction,
    pub distance: usize,
}

pub type WireInstruction = Vec<Instruction>;
pub type Wire = Vec<Point>;
pub type WireWithSteps = Vec<PointWithSteps>;

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        // e.g. R990
        let mut chars = value.trim().chars();
        let direction = Direction::from(chars.next().unwrap());
        let distance: usize = chars.collect::<String>().parse().unwrap();
        Self {
            direction,
            distance,
        }
    }
}

pub trait ToWire {
    fn to_wire(&self) -> Wire;
    fn to_wire_with_steps(&self) -> WireWithSteps;
}

impl ToWire for WireInstruction {
    fn to_wire(&self) -> Wire {
        self.iter()
            .fold(
                (Wire::new(), Point::new()),
                |(mut wire, mut point), instruction| {
                    (0..instruction.distance).for_each(|_| {
                        point.step(instruction.direction);
                        wire.push(point);
                    });
                    (wire, point)
                },
            )
            .0
    }

    fn to_wire_with_steps(&self) -> WireWithSteps {
        self.iter()
            .fold(
                (WireWithSteps::new(), PointWithSteps::new()),
                |(mut wire, mut point), instruction| {
                    (0..instruction.distance).for_each(|_| {
                        point.step(instruction.direction);
                        wire.push(point);
                    });
                    (wire, point)
                },
            )
            .0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn instruction_from_str_success_up() {
        assert_eq!(
            Instruction::from("U123"),
            Instruction {
                direction: Direction::Up,
                distance: 123
            }
        );
    }
    #[test]
    fn instruction_from_str_success_right() {
        assert_eq!(
            Instruction::from("R123"),
            Instruction {
                direction: Direction::Right,
                distance: 123
            }
        );
    }
    #[test]
    fn instruction_from_str_success_down() {
        assert_eq!(
            Instruction::from("D123"),
            Instruction {
                direction: Direction::Down,
                distance: 123
            }
        );
    }
    #[test]
    fn instruction_from_str_success_left() {
        assert_eq!(
            Instruction::from("L123"),
            Instruction {
                direction: Direction::Left,
                distance: 123
            }
        );
    }
    #[test]
    #[should_panic]
    fn instruction_from_str_failure_invalid_usize() {
        let _ = Instruction::from("uabc");
    }
    #[test]
    fn to_wire_test() {
        let wire_instruction = WireInstruction::from([
            Instruction {
                direction: Direction::Up,
                distance: 3,
            },
            Instruction {
                direction: Direction::Right,
                distance: 2,
            },
        ]);
        let expected_wire = Wire::from([
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 0, y: 3 },
            Point { x: 1, y: 3 },
            Point { x: 2, y: 3 },
        ]);
        let resulting_wire = wire_instruction.to_wire();
        assert_eq!(expected_wire, resulting_wire);
    }
}

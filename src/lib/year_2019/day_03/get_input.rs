use crate::year_2019::day_03::instruction::{Instruction, WireInstruction};

pub fn get_input() -> (WireInstruction, WireInstruction) {
    let mut wires = include_str!("input.txt").lines().map(|wire| {
        wire.split(',')
            .map(Instruction::from)
            .collect()
    });
    (wires.next().unwrap(), wires.next().unwrap())
}

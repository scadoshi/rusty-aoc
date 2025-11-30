use crate::year_2019::day_03::instruction::{Instruction, WireInstruction};

pub fn get_input() -> (WireInstruction, WireInstruction) {
    let mut wires = include_str!("input.txt").lines().map(|wire| {
        wire.split(',')
            .map(|instr| Instruction::from(instr))
            .collect()
    });
    (wires.next().unwrap(), wires.next().unwrap())
}

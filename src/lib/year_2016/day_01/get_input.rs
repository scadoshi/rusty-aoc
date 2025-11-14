use crate::year_2016::day_01::models::Instruction;

#[allow(dead_code)]
pub fn get_input() -> Vec<Instruction> {
    include_str!("input.txt")
        .split(", ")
        .map(|x| Instruction::from(x.trim()))
        .collect()
}

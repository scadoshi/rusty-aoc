use aoc::{
    run::{Run, funbox},
    year_2024::day_05::{get_input::get_input, part_01::part_01, part_02::part_02},
};

pub fn main() {
    let input = get_input();
    let functions = vec![funbox("Part 01", part_01), funbox("Part 02", part_02)];
    functions.run(&input);
}

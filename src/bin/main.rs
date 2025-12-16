use aoc::{
    run::{Run, funbox},
    year_2016::day_04::*,
};

pub fn main() {
    let started_at = std::time::Instant::now();
    let input = get_input();
    let functions = vec![funbox("Part 01", part_01), funbox("Part 02", part_02)];
    functions.run(&input);
    println!("\ntotal runtime: {:?}\n", started_at.elapsed());
}

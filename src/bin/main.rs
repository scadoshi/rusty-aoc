use aoc::{
    run::{funbox, Run},
    year_2020::day_04::*,
};

pub fn main() {
    let start = std::time::Instant::now();
    let input = get_input();
    println!("# Input parsing: {:?}", start.elapsed());
    let functions = [funbox("Part 01", part_01), funbox("Part 02", part_02)];
    functions.run(&input);
    println!("# Total runtime: {:?}\n", start.elapsed());
}

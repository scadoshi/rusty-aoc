use aoc::year_2015::day_03::{get_input::get_input, part_01::part_01, part_02::part_02};

pub fn main() {
    let input = get_input();

    let mut started_at = std::time::Instant::now();
    let result = part_01(&input);
    println!("part_01={:?}, run_time={:?}", result, started_at.elapsed());

    started_at = std::time::Instant::now();
    let result = part_02(&input);
    println!("part_02={:?}, run_time={:?}", result, started_at.elapsed());
}

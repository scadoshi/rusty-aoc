use aoc::year_2024::day_01::{get_input::get_input, part_01::part_01, part_02::part_02};

fn main() {
    let input = get_input();

    let mut started_at = std::time::Instant::now();
    println!(
        "part_01={:?}, run_time={:?}",
        part_01(&input),
        started_at.elapsed()
    );

    started_at = std::time::Instant::now();
    println!(
        "part_02={:?}, run_time={:?}",
        part_02(&input),
        started_at.elapsed()
    );
}

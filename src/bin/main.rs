use aoc::{
    time_run::TimeRun,
    year_2024::day_05::{get_input::get_input, part_01::part_01, part_02::part_02},
};

pub fn main() {
    let input = get_input();
    let functions = vec![
        (
            "Part 01".to_string(),
            Box::new(part_01) as Box<dyn Fn(_) -> _>,
        ),
        (
            "Part 02".to_string(),
            Box::new(part_02) as Box<dyn Fn(_) -> _>,
        ),
    ];
    functions.time_run(&input);
}

use crate::year_2025::day_08::euclidean_point::Point;

pub fn get_input() -> Vec<Point> {
    include_str!("input.txt")
        .lines()
        .map(|l| Point::from(l))
        .collect()
}

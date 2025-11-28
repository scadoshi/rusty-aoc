use crate::year_2016::day_03::triangle::Triangle;

pub fn get_input() -> Vec<Triangle> {
    include_str!("input.txt")
        .lines()
        .map(|x| Triangle::try_from(x).unwrap())
        .collect()
}

use crate::year_2016::day_03::triangle::Triangle;

pub fn part_01(input: &[Triangle]) -> usize {
    input.iter().filter(|t| t.is_valid()).count()
}

use crate::year_2016::day_03::{pivot::Pivot, triangle::Triangle};

pub fn part_02(input: &[Triangle]) -> usize {
    input.pivot().iter().filter(|t| t.is_valid()).count()
}

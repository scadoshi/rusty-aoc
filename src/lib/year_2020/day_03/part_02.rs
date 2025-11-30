use crate::year_2020::day_03::collisions_at_slope::CollisionsAtSlope;

pub fn part_02(input: &[Vec<char>]) -> usize {
    input.collisions_at_slope(1, 1)
        * input.collisions_at_slope(1, 3)
        * input.collisions_at_slope(1, 5)
        * input.collisions_at_slope(1, 7)
        * input.collisions_at_slope(2, 1)
}

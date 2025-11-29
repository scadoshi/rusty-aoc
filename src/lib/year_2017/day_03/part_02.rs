use crate::year_2017::day_03::point::SpiralingPoint as Spoint;
use std::collections::HashSet;

pub fn part_02(input: &usize) -> usize {
    let mut spoint = Spoint::new();
    let mut visited: HashSet<Spoint> = HashSet::from([spoint]);
    loop {
        for _ in 0..2 {
            for _ in 0..spoint.steps {
                spoint.step();
                spoint.value = spoint.sum_of_surrounding_point_values(&visited);
                if spoint.value >= *input {
                    return spoint.value;
                }
                visited.insert(spoint);
            }
            spoint.turn_left();
        }
        spoint.steps += 1;
    }
}

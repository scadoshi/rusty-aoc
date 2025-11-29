use crate::year_2017::day_03::point::SpiralingPoint as Spoint;
use std::collections::HashSet;

pub fn part_01(input: &usize) -> usize {
    let mut spoint = Spoint::new();
    let mut visited: HashSet<Spoint> = HashSet::from([spoint]);
    loop {
        for _ in 0..2 {
            for _ in 0..spoint.steps {
                spoint.step();
                if spoint.value == *input {
                    return spoint.point.distance_from_origin();
                }
                visited.insert(spoint);
            }
            spoint.turn_left();
        }
        spoint.steps += 1;
    }
}

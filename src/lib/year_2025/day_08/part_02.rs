use crate::year_2025::day_08::euclidean_point::{DistanceSortedPointCombinations, Point};
use std::collections::HashSet;

pub fn part_02(input: &[Point]) -> usize {
    let mut distances = input.distance_sorted_point_combinations();
    let mut last_p1 = None::<Point>;
    let mut last_p2 = None::<Point>;
    let mut circuits = Vec::<HashSet<Point>>::new();
    while let Some((p1, p2)) = distances.pop() {
        let matches: Vec<&mut HashSet<Point>> = circuits
            .iter_mut()
            .filter(|c| c.contains(&p1) || c.contains(&p2))
            .collect();
        let len = matches.len();
        assert!(len < 3);
        if len == 2 {
            let mut iter = matches.into_iter();
            let (c1, c2) = (iter.next().unwrap(), iter.next().unwrap());
            c1.extend(c2.iter());
            c2.clear();
            circuits.retain(|c| !c.is_empty());
        } else if len == 1 {
            let c1 = matches.into_iter().next().unwrap();
            c1.extend([p1, p2]);
        } else {
            circuits.push(HashSet::from([p1, p2]));
        }
        last_p1 = Some(p1);
        last_p2 = Some(p2);
        if circuits.iter().any(|c| c.len() == input.len()) {
            break;
        }
    }
    usize::try_from(last_p1.unwrap().x * last_p2.unwrap().x).unwrap()
}

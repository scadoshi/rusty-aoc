use crate::year_2025::day_08::euclidean_point::{DistanceSortedPointCombinations, Point};
use std::collections::HashSet;

pub fn part_01(input: &[Point]) -> usize {
    let mut distances = input.distance_sorted_point_combinations();
    let mut circuit_lengths: Vec<usize> = (0..1000)
        .fold(Vec::<HashSet<Point>>::new(), |mut circuits, _| {
            if let Some((p1, p2)) = distances.pop() {
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
            }
            circuits
        })
        .iter()
        .map(|c| c.len())
        .collect();
    circuit_lengths.sort();
    circuit_lengths.iter().rev().take(3).product()
}

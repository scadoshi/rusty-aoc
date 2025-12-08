use crate::year_2025::day_08::euclidean_point::Point;
use std::collections::{HashSet, VecDeque};

pub fn part_01(input: &[Point]) -> usize {
    let mut distances: Vec<(f32, HashSet<Point>)> = input
        .iter()
        .map(|p1| {
            input
                .iter()
                .filter(|p| *p != p1)
                .map(|p2| (p1.distance_to(*p2), HashSet::from([*p1, *p2])))
                .collect::<Vec<(f32, HashSet<Point>)>>()
        })
        .flatten()
        .collect();
    distances.sort_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).unwrap());
    let mut queue: VecDeque<(f32, HashSet<Point>)> = distances.into_iter().collect();
    let mut circuit_lens: Vec<usize> = (0..10)
        .fold(Vec::<HashSet<Point>>::new(), |mut circuits, _| {
            if let Some((_, point_set)) = queue.pop_front() {
                if let Some(circuit) = circuits
                    .iter_mut()
                    .find(|c| point_set.iter().any(|p| c.contains(p)))
                {
                    circuit.extend([p1, p2]);
                } else {
                    circuits.push(HashSet::from([p1, p2]));
                }
            }
            println!("{:?}", circuits);
            circuits
        })
        .iter()
        .map(|c| c.len())
        .collect();
    circuit_lens.sort();
    circuit_lens.iter().rev().take(3).product()
}

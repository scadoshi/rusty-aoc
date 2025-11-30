use crate::year_2018::day_03::{claim::Claim, point::Point};

pub fn part_02(input: &[Claim]) -> Option<usize> {
    let claims: Vec<(usize, Vec<Point>)> = input.iter().fold(Vec::new(), |mut claims, claim| {
        claims.push((claim.id, claim.to_points()));
        claims
    });
    for (id, points) in claims.iter() {
        if !claims
            .iter()
            .filter(|(other_id, _)| id != other_id)
            .any(|(_, other_points)| points.iter().any(|point| other_points.contains(point)))
        {
            return Some(*id);
        }
    }
    None
}

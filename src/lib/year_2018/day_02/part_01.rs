use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_01(input: &[String]) -> i32 {
    let (twins, triplets) = input
        .iter()
        .fold((0, 0), |(mut twins, mut triplets), line| {
            let map: HashMap<char, i32> = line.chars().fold(HashMap::new(), |mut map, char| {
                *map.entry(char).or_default() += 1;
                map
            });

            if map.iter().any(|(_, v)| *v == 2) {
                twins += 1;
            }

            if map.iter().any(|(_, v)| *v == 3) {
                triplets += 1;
            }

            (twins, triplets)
        });
    twins * triplets
}

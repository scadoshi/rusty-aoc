use std::{collections::HashSet, ops::RangeInclusive};

pub fn part_02(input: &[RangeInclusive<usize>]) -> usize {
    let mut invalid_cache = HashSet::<usize>::new();
    let mut total = 0;
    for range in input {
        'ids: for id in range.clone() {
            if invalid_cache.contains(&id) {
                total += id;
                continue;
            }
            for chunk_size in 1..=id.to_string().len() / 2 {
                let chunks: Vec<String> = id
                    .to_string()
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(chunk_size)
                    .map(|chunk| chunk.iter().collect())
                    .collect();
                if chunks.iter().all(|chunk| *chunk == chunks[0]) {
                    total += id;
                    invalid_cache.insert(id);
                    continue 'ids;
                }
            }
        }
    }
    total
}

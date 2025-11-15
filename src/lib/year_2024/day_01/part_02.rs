use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_02(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (vec1, vec2) = input;
    let map1: HashMap<i32, i32> = vec1.into_iter().fold(HashMap::new(), |mut map, num| {
        *map.entry(*num).or_default() += 1;
        map
    });
    let map2: HashMap<i32, i32> = vec2.into_iter().fold(HashMap::new(), |mut map, num| {
        *map.entry(*num).or_default() += 1;
        map
    });
    map1.into_iter()
        .map(|(num, count1)| {
            let count2 = *map2.get(&num).unwrap_or(&0);
            num * count1 * count2
        })
        .sum()
}

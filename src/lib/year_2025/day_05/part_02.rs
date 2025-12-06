pub fn part_02(input: &(Vec<(usize, usize)>, Vec<usize>)) -> usize {
    let (mut ranges, _) = input.clone();
    ranges.sort_by(|(a, _), (b, _)| a.cmp(b));
    let mut merged = vec![ranges.remove(0)];
    while !ranges.is_empty() {
        let last = merged.len() - 1;
        let (start1, end1) = merged.get_mut(last).unwrap();
        let (start2, end2) = ranges.remove(0);
        if start2 >= *start1 && start2 <= *end1 && end2 >= *start1 && end2 <= *end1 {
            continue;
        } else if start2 >= *start1 && start2 <= *end1 {
            *end1 = end2;
        } else {
            merged.push((start2, end2));
        }
    }
    merged
        .into_iter()
        .map(|(start, end)| (end - start) + 1)
        .sum()
}

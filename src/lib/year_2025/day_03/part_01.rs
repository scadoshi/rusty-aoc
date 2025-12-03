pub fn part_01(input: &[Vec<u8>]) -> usize {
    input.iter().fold(0, |total, nums| {
        let (i, max) = nums
            .iter()
            .enumerate()
            .rev()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();
        let (left, right);
        if i == nums.len() - 1 {
            left = *nums[..i].iter().max().unwrap();
            right = *max;
        } else {
            left = *max;
            right = *nums[i + 1..].iter().max().unwrap();
        }
        total
            + (left.to_string() + right.to_string().as_ref())
                .parse::<usize>()
                .unwrap()
    })
}

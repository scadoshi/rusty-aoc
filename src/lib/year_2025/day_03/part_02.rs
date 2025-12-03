pub const MAX_BATTERIES: usize = 12;

pub fn part_02(input: &[Vec<u8>]) -> usize {
    input.iter().fold(0, |mut total, nums| {
        let mut on = nums.clone();
        while on.len() > MAX_BATTERIES {
            todo!()
        }
        total
    })
}

use crate::year_2020::day_01::part_01::two_sum;

pub fn three_sum(nums: &[i32], target: i32) -> Option<(i32, i32, i32)> {
    for num1 in nums.iter() {
        let sub_target = target - num1;
        if let Some((num2, num3)) = two_sum(nums, sub_target) {
            return Some((*num1, num2, num3));
        }
    }
    None
}

#[allow(dead_code)]
pub fn part_02(input: &[i32]) -> Option<i32> {
    three_sum(input, 2020).map(|(num1, num2, num3)| num1 * num2 * num3)
}

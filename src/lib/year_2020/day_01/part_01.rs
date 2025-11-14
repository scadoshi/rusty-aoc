#[allow(dead_code)]
pub fn two_sum(nums: &[i32], target: i32) -> Option<(i32, i32)> {
    for num1 in nums {
        let sub_target = target - num1;
        if let Some(num2) = nums.iter().find(|num2| **num2 == sub_target) {
            return Some((*num1, *num2));
        }
    }
    None
}

#[allow(dead_code)]
pub fn part_01(input: &[i32]) -> Option<i32> {
    two_sum(input, 2020).map(|(num1, num2)| num1 * num2)
}

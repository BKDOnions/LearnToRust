use crate::algorithms::solution::Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        for index in 1..nums.len() / 2 + 1 {
            if nums[index] < min {
                min = nums[index];
            }
            if nums[nums.len() - index] < min {
                min = nums[nums.len() - index];
            }
        }
        return min;
    }
}
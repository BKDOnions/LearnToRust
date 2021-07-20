use crate::algorithms::solution::Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        nums.sort();
        let mut max = nums[0] + nums[len - 1];
        for index in 1..len / 2 {
            let temp = nums[index] + nums[len - 1 - index];
            if max < temp {
                max = temp;
            }
        }
        return max;
    }
}
use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut dup_checker = HashSet::new();
        for num in nums {
            if !dup_checker.insert(num) {
                return num;
            }
        }
        0
    }
}

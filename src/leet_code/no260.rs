use std::collections::HashSet;

use crate::algorithms::solution::Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();
        for x in nums {
            if !set.insert(x) {
                set.remove(&x);
            }
        }
        set.drain().collect()
    }
}

use std::collections::HashSet;

use crate::algorithms::solution::Solution;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let max = candy_type.len() / 2;
        if max == 0 || max == 1 {
            return max as i32;
        }
        let mut res_set: HashSet<i32> = HashSet::new();
        for candy in candy_type {
            res_set.insert(candy);
            if res_set.len() >= max {
                return max as i32;
            }
        }
        return res_set.len() as i32;
    }
}

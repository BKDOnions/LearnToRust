use crate::Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        let mut counter: Vec<(i32, usize)> = Vec::new();
        nums.reverse();
        for (index, &value) in nums.iter().enumerate() {
            let insert_index = search(&counter, value, 0, counter.len());
            counter.insert(insert_index, (value, index));
        }

        return 0;
    }
}

fn search(nums: &Vec<(i32, usize)>, target: i32, low: usize, high: usize) -> usize {
    if high < low {
        return low;
    }
    let mid = (low + high) >> 1;

    if nums[mid].0 == target {
        return mid;
    }
    if nums[mid].0 > target {
        return if mid > 0 {
            search(nums, target, low, mid - 1)
        } else {
            mid
        };
    }
    if nums[mid].0 < target {
        return search(nums, target, mid + 1, high);
    }
    return 0;
}

#[cfg(test)]
mod test_offer_51 {
    use crate::Solution;
    use std::collections::HashMap;

    #[test]
    fn test_case() {
        let mut res = HashMap::new();
        res.insert(1, 2);
        res.insert(3, 4);
        for (x, (key, value)) in res.iter().enumerate() {
            println!("x: {} key {} value: {}", x, key, value);
        }
        // let nums = vec![];
        // println!("{}", Solution::reverse_pairs(nums));
    }
}

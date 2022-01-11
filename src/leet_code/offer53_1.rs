use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(idx) => {
                return Solution::count(nums, idx);
            }
            Err(_) => 0,
        }
    }
    fn count(nums: Vec<i32>, idx: usize) -> i32 {
        let mut counter = 1;
        let mut prior = idx;
        let mut after = idx + 1;
        while prior > 0 {
            prior -= 1;
            if nums[prior] == nums[idx] {
                counter += 1;
            }
        }
        while after < nums.len() {
            if nums[after] == nums[idx] {
                counter += 1;
            }
            after += 1;
        }
        return counter;
    }
}

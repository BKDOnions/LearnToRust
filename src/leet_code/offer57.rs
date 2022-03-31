use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for &num in &nums {
            match nums.binary_search(&(target - num)) {
                Ok(x) => return vec![num, nums[x]],
                Err(_) => {
                    continue;
                }
            }
        }
        return Vec::new();
    }
}

#[cfg(test)]
mod test_offer57 {
    use crate::Solution;

    #[test]
    fn test_case() {
        println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
    }
}

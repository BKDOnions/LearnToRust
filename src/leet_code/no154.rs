use crate::algorithms::solution::Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        if length == 1 {
            return nums[0];
        }
        for index in 0..length {
            if index >= length - index {
                break;
            }
            if nums[index] > nums[index + 1] {
                return nums[index + 1];
            }
            if nums[length - index - 1] < nums[length - index - 2] {
                return nums[length - index - 1];
            }
        }
        return nums[0];
    }
}

#[cfg(test)]
mod test_154 {
    use crate::Solution;

    #[test]
    fn test_case() {
        let x = vec![4, 5, 6, 7, 0, 1, 4];
        assert_eq!(Solution::find_min(x), 0);
    }
}

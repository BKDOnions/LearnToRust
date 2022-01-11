use crate::Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low < high {
            let mid = (low + high) >> 1;
            if nums[mid] == mid as i32 {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        return if low == nums.len() - 1 && nums[low] == low as i32 {
            low + 1
        } else {
            low
        } as i32;
    }
}

#[cfg(test)]
mod offer53_2 {
    use crate::Solution;

    #[test]
    fn test() {
        let nums = vec![0, 1, 2, 3, 4, 5, 7, 8, 9];
        assert_eq!(Solution::missing_number(nums), 6);
    }
}

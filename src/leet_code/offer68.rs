use crate::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> usize {
        search(&nums, target, 0, nums.len() - 1)
    }
}

fn search(nums: &Vec<i32>, target: i32, low: usize, high: usize) -> usize {
    if high < low {
        return low;
    }
    let mid = (low + high) >> 1;

    if nums[mid] == target {
        return mid;
    }
    if nums[mid] > target {
        return if mid > 0 {
            search(nums, target, low, mid - 1)
        } else {
            mid
        };
    }
    if nums[mid] < target {
        return search(nums, target, mid + 1, high);
    }
    return 0;
}

#[cfg(test)]
mod test_offer_68 {
    use crate::Solution;

    #[test]
    fn test_case() {
        let nums = vec![1, 3, 5, 6];
        println!("{}", Solution::search_insert(nums, 2));
    }
}

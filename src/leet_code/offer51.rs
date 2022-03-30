use crate::Solution;

impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        merge_sort(&mut nums)
    }
}

pub fn merge_sort(nums: &mut Vec<i32>) -> i32 {
    fn _merge(nums: &mut Vec<i32>, left: usize, mid: usize, right: usize) -> i32 {
        let mut counter = 0;
        let left_part: Vec<i32> = nums[left..mid].iter().cloned().collect();
        let right_part: Vec<i32> = nums[mid..right].iter().cloned().collect();
        let (mut left_offset, mut right_offset) = (0usize, 0usize);
        while left_offset < left_part.len() || right_offset < right_part.len() {
            if right_offset == right_part.len()
                || (left_offset < left_part.len()
                    && left_part[left_offset] <= right_part[right_offset])
            {
                nums[left + left_offset + right_offset] = left_part[left_offset];
                left_offset += 1;
            } else if left_offset == mid + 1 {
                nums[left + left_offset + right_offset] = right_part[right_offset];
                right_offset += 1;
            } else {
                nums[left + left_offset + right_offset] = right_part[right_offset];
                right_offset += 1;
                counter += (mid - left_offset + 1) as i32;
            }
        }
        return counter;
    }

    fn _merge_sort(nums: &mut Vec<i32>, left: usize, right: usize) -> i32 {
        let mut counter = 0;
        if left + 1 < right {
            let mid = (left + right) / 2;
            counter += _merge_sort(nums, left, mid);
            counter += _merge_sort(nums, mid, right);
            counter += _merge(nums, left, mid, right);
        }
        return counter;
    }
    return _merge_sort(nums, 0, nums.len());
}

#[cfg(test)]
mod test_offer_51 {
    use crate::Solution;

    #[test]
    fn test_case() {
        let nums = vec![7, 5, 6, 4];
        println!("{}", Solution::reverse_pairs(nums));
    }
}

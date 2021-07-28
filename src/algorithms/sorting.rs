use std::collections::VecDeque;

use crate::algorithms::solution::Solution;

impl Solution {
    pub fn insert_sort(nums: &mut Vec<usize>) {
        for i in 1..nums.len() {
            let (mut p, v) = (i, nums[i]);
            while p > 0 && nums[p - 1] > v {
                nums[p] = nums[p - 1];
                p -= 1;
            }
            nums[p] = v;
        }
    }

    pub fn shell_sort(nums: &mut Vec<usize>) {
        fn _insert_sort(nums: &mut Vec<usize>, g: usize) {
            for i in g..nums.len() {
                let (mut p, v) = (i, nums[i]);
                while p >= g && nums[p - g] > v {
                    nums[p] = nums[p - g];
                    p -= g;
                }
                nums[p] = v;
            }
        }

        let mut a: VecDeque<usize> = VecDeque::new();
        a.push_front(1);
        while *a.front().unwrap() <= nums.len() {
            a.push_front(3 * a.front().unwrap() + 1);
        }
        for &g in a.iter() {
            _insert_sort(nums, g);
        }
    }

    pub fn selection_sort(nums: &mut Vec<usize>) {
        for i in 0..nums.len() - 1 {
            let mut p = i;
            for j in i + 1..nums.len() {
                if nums[j] < nums[p] {
                    p = j;
                }
            }
            nums.swap(i, p);
        }
    }
    pub fn quick_sort(nums: &mut Vec<usize>) {
        fn _partition(nums: &mut Vec<usize>, begin: usize, end: usize) -> usize {
            let (mut i, v) = (begin, nums[end - 1]);
            for j in begin..end - 1 {
                if nums[j] <= v {
                    nums.swap(i, j);
                    i += 1;
                }
            }
            nums.swap(i, end - 1);
            i
        }
        fn _quick_sort(nums: &mut Vec<usize>, begin: usize, end: usize) {
            if begin + 1 < end {
                let mid = _partition(nums, begin, end);
                _quick_sort(nums, begin, mid);
                _quick_sort(nums, mid + 1, end);
            }
        }
        _quick_sort(nums, 0, nums.len())
    }

    pub fn merge_sort(nums: &mut Vec<usize>) {
        fn _merge(nums: &mut Vec<usize>, left: usize, mid: usize, right: usize) {
            let left_part: Vec<usize> = nums[left..mid].iter().cloned().collect();
            let right_part: Vec<usize> = nums[mid..right].iter().cloned().collect();
            let (mut left_offset, mut right_offset) = (0usize, 0usize);
            while left_offset < left_part.len() || right_offset < right_part.len() {
                if right_offset == right_part.len()
                    || (left_offset < left_part.len() && left_part[left_offset] <= right_part[right_offset]) {
                    nums[left + left_offset + right_offset] = left_part[left_offset];
                    left_offset += 1;
                } else {
                    nums[left + left_offset + right_offset] = right_part[right_offset];
                    right_offset += 1;
                }
            }
        }

        fn _merge_sort(nums: &mut Vec<usize>, left: usize, right: usize) {
            if left + 1 < right {
                let mid = (left + right) / 2;
                _merge_sort(nums, left, mid);
                _merge_sort(nums, mid, right);
                _merge(nums, left, mid, right);
            }
        }
        _merge_sort(nums, 0, nums.len())
    }
}
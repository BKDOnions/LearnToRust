use crate::algorithms::solution::Solution;

pub struct MountainArray {
    pub arr: Vec<i32>,
}

impl MountainArray {
    fn get(&self, index: i32) -> i32 {
        if index as usize > self.arr.len() - 1 || index < 0 {
            -1
        } else {
            self.arr[index as usize]
        }
    }
    fn length(&self) -> i32 {
        self.arr.len() as i32
    }
}

impl Solution {
    pub fn find_in_mountain_array(target: i32, mountainArr: &MountainArray) -> i32 {
        let top = Solution::find_mountain_top(mountainArr, 0, mountainArr.length() - 1);
        let left = Solution::find_left(mountainArr, target, 0, top);
        return if left != -1 {
            left
        } else {
            Solution::find_right(mountainArr, target, top, mountainArr.length() - 1)
        };
    }
    fn find_mountain_top(mountainArray: &MountainArray, start: i32, end: i32) -> i32 {
        let mid = mountainArray.get((start + end) / 2);
        let prior = mountainArray.get((start + end) / 2 - 1);
        let next = mountainArray.get((start + end) / 2 + 1);
        return if prior < mid && mid < next {
            Solution::find_mountain_top(mountainArray, ((start + end) / 2), end)
        } else if prior > mid && mid > next {
            Solution::find_mountain_top(mountainArray, start, ((start + end) / 2))
        } else {
            (start + end) / 2
        };
    }
    fn find_left(mountainArray: &MountainArray, target: i32, start: i32, end: i32) -> i32 {
        let mid = mountainArray.get((start + end) / 2);
        return if start <= end {
            if mid > target {
                Solution::find_left(mountainArray, target, start, (start + end) / 2 - 1)
            } else if mid < target {
                Solution::find_left(mountainArray, target, (start + end) / 2 + 1, end)
            } else {
                (start + end) / 2
            }
        } else {
            -1
        };

    }
    fn find_right(mountainArray: &MountainArray, target: i32, start: i32, end: i32) -> i32 {
        let mid = mountainArray.get((start + end) / 2);
        return if start <= end {
            if mid > target {
                Solution::find_right(mountainArray, target, (start + end) / 2 + 1, end)
            } else if mid < target {
                Solution::find_right(mountainArray, target, start, (start + end) / 2 - 1)
            } else {
                (start + end) / 2
            }
        } else {
            -1
        };
    }
}
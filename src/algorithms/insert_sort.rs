//! 1) traverse j from 2 to arr.length()
//!
//! 2)
use crate::algorithms::solution::Solution;

impl Solution {
    pub fn insert_sort(arr: &mut Vec<i32>) {
        if arr.len() <= 1 {
            return;
        } else {
            for mut j in 1..arr.len() {
                let mut key = arr[j];
                while j >= 1 && arr[j - 1] > key {
                    arr[j] = arr[j - 1];
                    j -= 1;
                }
                arr[j] = key;
            }
            return;
        }
    }
}

use crate::Solution;
impl Solution {
    pub fn merge_sort(arr: &mut Vec<i32>, start: &usize, end: &usize) {
        if start < end {
            let mid = (start + end) / 2;
            merge_sort(arr, start, mid);
            merge_sort(arr, mid + 1, end);
            merge(arr, start, &mid, end);
        }
    }
}

fn merge(arr: &mut Vec<i32>, start: &usize, mid: &usize, end: &usize) {
    let begin_point1 = mid - start + 1;
    let begin_point2 = end - mid;
    let left = arr[begin_point1..mid];
    let right = arr[begin_point2 + 1..end];
}

use crate::algorithms::solution::Solution;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_whitespace().collect::<Vec<_>>().len() as i32
    }
}

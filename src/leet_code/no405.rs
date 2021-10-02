use crate::algorithms::solution::Solution;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        format!("{:x}", num)
    }
}
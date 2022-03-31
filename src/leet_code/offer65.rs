use crate::Solution;

impl Solution {
    pub fn add(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        let sum = a ^ b;
        let carry = (a & b) << 1;
        return Solution::add(sum, carry);
    }
}

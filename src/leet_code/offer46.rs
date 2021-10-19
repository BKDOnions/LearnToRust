use crate::algorithms::solution::Solution;

static mut COUNTER: i32 = 0;

impl Solution {
    pub fn translate_num(mut num: i32) -> i32 {
        unsafe {
            COUNTER = 0;
        }
        // Get Digits
        let mut digits: Vec<i32> = Vec::new();
        while num > 0 {
            digits.push(num % 10);
            num = num / 10;
        }
        digits.reverse();

        //Dfs
        Solution::dfs(0, &digits);
        unsafe {
            return COUNTER;
        }
    }
    fn dfs(c_idx: usize, digits: &Vec<i32>) {
        if c_idx > digits.len() - 1 {
            unsafe {
                COUNTER += 1;
            }
            return;
        }
        Solution::dfs(c_idx + 1, digits);
        if c_idx + 1 < digits.len() && digits[c_idx] * 10 + digits[c_idx + 1] < 26 {
            Solution::dfs(c_idx + 2, digits);
        }
    }
}

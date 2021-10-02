use crate::algorithms::solution::Solution;

impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let word = word.as_bytes();
        let mut step: i32 = Solution::find_min_duration( 97, word[0]);
        for idx in 1..word.len() {
            step += Solution::find_min_duration(word[idx - 1], word[idx]);
        }
        step
    }
    fn find_min_duration(letter1: u8, letter2: u8) -> i32 {
        let letter1 = letter1 as i32;
        let letter2 = letter2 as i32;
        let forward = (letter2 - letter1).abs();
        let backward = 26 - forward;
        if forward > backward {
            backward + 1
        }else {
            forward + 1
        }
    }

}
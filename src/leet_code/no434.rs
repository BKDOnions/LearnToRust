use crate::algorithms::solution::Solution;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut counter = 0;
        let strings: Vec<&str> = s.split(" ").collect();
        for string in strings {
            if !"".eq(string) {
                counter += 1;
            }
        }
        return counter;
    }
}

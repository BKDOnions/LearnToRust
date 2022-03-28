use crate::Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let to_bin = format!("{:b}", n);
        !to_bin.contains("00") && !to_bin.contains("11")
    }
}

#[cfg(test)]
mod test_693 {
    use crate::Solution;

    #[test]
    fn test_case() {
        assert_eq!(Solution::has_alternating_bits(21), true);
    }
}

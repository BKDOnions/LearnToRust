use crate::algorithms::solution::Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for i in 1..n + 1 {
            let a = i % 3;
            let b = i % 5;
            if a == 0 && b == 0 {
                res.push("FizzBuzz".to_string());
            } else if a == 0 {
                res.push("Fizz".to_string());
            } else if b == 0 {
                res.push("Buzz".to_string());
            } else {
                res.push(i.to_string());
            }
        }
        res
    }
}

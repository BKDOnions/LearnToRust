use crate::algorithms::solution::Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for i in 1..n + 1 {
            match (i % 3, i % 5) {
                (0, 0) => res.push(String::from("FizzBuzz")),
                (0, _) => res.push(String::from("Fizz")),
                (_, 0) => res.push(String::from("Buzz")),
                _ => res.push(format!("{}", i)),
            }
        }
        res
    }
}

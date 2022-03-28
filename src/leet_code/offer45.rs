use crate::Solution;

impl Solution {
    pub fn min_number(mut nums: Vec<i32>) -> String {
        let mut res = String::new();
        nums.sort_by(|&x, &y| {
            let mut str1 = x.to_string();
            let mut str2 = y.to_string();
            str1.push_str(&y.to_string());
            str2.push_str(&x.to_string());
            str1.cmp(&str2)
        });
        for num in nums {
            res.push_str(&num.to_string())
        }
        res
    }
}

#[cfg(test)]
mod test_offer45 {
    use crate::Solution;

    #[test]
    fn test_case() {
        println!("{}", Solution::min_number(vec![3, 30, 34, 5, 9]));
    }
}

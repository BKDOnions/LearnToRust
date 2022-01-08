use crate::Solution;

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let n = if n == 0 { 0 } else { n - 1 } as usize;
        let mut pre = String::from(&s[..n]);
        let post = String::from(&s[n..]);
        pre.push_str(&post);
        pre
    }
}

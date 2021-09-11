use crate::algorithms::solution::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        for (count, unicode) in strs[0].as_bytes().iter().enumerate() {
            for str in strs.iter() {
                if str.is_empty() {
                    return "".to_string();
                }
                if count > str.len() - 1 {
                    return String::from(&strs[0][0..count]);
                }
                if str.as_bytes()[count].ne(unicode) {
                    return String::from(&strs[0][0..count]);
                }
            }
        }
        strs[0].to_string()
    }
}

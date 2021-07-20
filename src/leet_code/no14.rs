use crate::algorithms::solution::Solution;

impl Solution{
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let str1 = String::from(&strs[0]).as_bytes();
        for (count, unicode) in str1.iter().enumerate(){
            for str in strs.iter() {
                if str.as_bytes()[count].ne(unicode) {
                    str1[0..count].to_owned()
                }
            }
        }
        "".to_string()
    }
}